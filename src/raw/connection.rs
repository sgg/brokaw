use std::fmt;
use std::io;
use std::io::{ErrorKind, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::str::FromStr;
use std::time::Duration;

use log::*;
use native_tls::TlsConnector;

use crate::raw::compression::{Compression, Decoder};
use crate::raw::error::Result;
use crate::raw::parse::{is_end_of_datablock, parse_data_block_line, parse_first_line};
use crate::raw::response::{DataBlocks, RawResponse};
use crate::raw::stream::NntpStream;
use crate::types::command::NntpCommand;
use crate::types::prelude::*;

/// TLS configuration for an [`NntpConnection`]
#[derive(Clone)]
pub struct TlsConfig {
    connector: TlsConnector,
    domain: String,
}

impl TlsConfig {
    /// Create a `TlsConfig` for use with [`NntpConnections`](NntpConnection)
    ///
    /// The `domain` will be passed to [`TlsConnector::connect`] for certificate validation
    /// during any TLS handshakes.
    pub fn new(domain: String, connector: TlsConnector) -> Self {
        Self { connector, domain }
    }

    /// Create a `TlsConfig` with the system default TLS settings
    ///
    /// The `domain` will be used to validate server certs during any TLS handshakes.
    pub fn default_connector(domain: impl AsRef<str>) -> Result<Self> {
        let connector = TlsConnector::new()?;
        Ok(Self {
            connector,
            domain: domain.as_ref().to_string(),
        })
    }

    /// The [`TlsConnector`] associated with the config
    pub fn connector(&self) -> &TlsConnector {
        &self.connector
    }
}

impl fmt::Debug for TlsConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TlsConfig")
            .field("domain", &self.domain)
            .finish()
    }
}

/// A raw connection to an NNTP Server
///
/// `NntpConnection` essentially wraps a stream. It is responsible for serializing commands
/// and deserializing and parsing responses from the server.
///
/// `NntpConnection` DOES...
///
/// * Work very hard not to allocate while reading/parsing response
/// * Provide facilities for you to manage your own read buffers
/// * Guarantee that Nntp responses are *framed* properly (though not that they are semantically valid)
///
/// `NntpConnection` DOES NOT...
///
/// * Manage any of the stateful details of the connection such as server capabilities,
/// selected group, or selected articles.
/// * Perform detailed parsing of responses.
///
/// For a more ergonomic client please see the [`NntpClient`](crate::client::NntpClient).
///
/// ## Usage
///
/// Please note that NNTP is a STATEFUL protocol and the Connection DOES NOT maintain any information
/// about this state.
///
/// The [`command`](NntpConnection::command) method can be used perform basic command/receive operations
///
/// For finer grained control over I/O please see the following:
///
/// * [`send`](Self::send) & [`send_bytes`](Self::send_bytes) for writing commands
/// * [`read_response`](Self::read_response) & [`read_response_auto`](Self::read_response_auto)
///   for reading responses
/// ## Buffer Management
///
/// The connection maintains several internal buffers for reading responses.
/// These buffers may grow when reading large responses.
///
/// The buffer sizes can be tuned via [`ConnectionConfig`], and they can be reset to their
/// preconfigured size by calling [`NntpConnection::reset_buffers`].
///
/// ## Example: Getting Capabilities
///
/// ```no_run
/// use std::time::Duration;
/// use brokaw::types::prelude::*;
/// use brokaw::types::command as cmd;
/// use brokaw::raw::connection::NntpConnection;
///
/// fn main() -> Result<(), Box::<dyn std::error::Error>> {
/// let (mut conn, init_resp) = NntpConnection::connect(
///         ("news.mozilla.org", 119),
///         None,
///         Some(Duration::from_secs(5))
///     )?;
///     assert_eq!(init_resp.code(), ResponseCode::Known(Kind::PostingAllowed));
///     let resp = conn.command(&cmd::Capabilities)?;
///     let data_blocks = resp.data_blocks().unwrap();
///     assert_eq!(&resp.code(), &ResponseCode::Known(Kind::Capabilities));
///
///     let contains_version = data_blocks
///         .lines()
///         .map(|line| std::str::from_utf8(line).unwrap())
///         .any(|l| l.contains("VERSION"));
///
///     assert!(contains_version);
///     Ok(())
/// }
#[derive(Debug)]
pub struct NntpConnection {
    stream: BufNntpStream,
    first_line_buf: Vec<u8>,
    data_blocks_buf: Vec<u8>,
    config: ConnectionConfig,
}

impl NntpConnection {
    /// Connect to an NNTP server
    pub fn connect(
        addr: impl ToSocketAddrs,
        config: ConnectionConfig,
    ) -> Result<(Self, RawResponse)> {
        let ConnectionConfig {
            compression: _,
            tls_config,
            read_timeout,
            write_timeout: _,
            first_line_buf_size,
            data_blocks_buf_size,
        } = config.clone();

        trace!("Opening TcpStream...");
        let tcp_stream = TcpStream::connect(&addr)?;

        tcp_stream.set_read_timeout(read_timeout)?;

        let nntp_stream = if let Some(TlsConfig { connector, domain }) = tls_config.as_ref() {
            trace!("Wrapping TcpStream w/ TlsConnector");
            connector.connect(domain, tcp_stream)?.into()
        } else {
            trace!("No TLS config providing, continuing with plain text");
            tcp_stream.into()
        };

        let first_line_buf = Vec::with_capacity(first_line_buf_size);
        let data_blocks_buf = Vec::with_capacity(data_blocks_buf_size);

        let mut conn = Self {
            stream: io::BufReader::new(nntp_stream),
            first_line_buf,
            data_blocks_buf,
            config,
        };

        let initial_resp = conn.read_response_auto()?;

        Ok((conn, initial_resp))
    }

    /// Create an NntpConnection with the default configuration
    pub fn with_defaults(addr: impl ToSocketAddrs) -> Result<(Self, RawResponse)> {
        Self::connect(addr, Default::default())
    }

    /// Send a command to the server and read the response
    ///
    /// This function will:
    /// 1. Block while reading the response
    /// 2. Parse the response
    /// 2. This function *may* allocate depending on the size of the response
    pub fn command<C: NntpCommand>(&mut self, command: &C) -> Result<RawResponse> {
        self.send(command)?;
        let resp = self.read_response_auto()?;
        Ok(resp)
    }

    /// Send a command and specify whether the response is multiline
    pub fn command_multiline<C: NntpCommand>(
        &mut self,
        command: &C,
        is_multiline: bool,
    ) -> Result<RawResponse> {
        self.send(command)?;
        let resp = self.read_response(Some(is_multiline))?;
        Ok(resp)
    }

    /// Send a command to the server, returning the number of bytes written
    ///
    /// The caller is responsible for reading the response
    pub fn send<C: NntpCommand>(&mut self, command: &C) -> Result<usize> {
        let bytes = self.send_bytes(command.to_string().as_bytes())?;
        Ok(bytes)
    }

    /// Send a command to the server, returning the number of bytes written
    ///
    /// This function can be used for commands not implemented/supported by the library
    /// (e.g. `LISTGROUP misc.test 3000238-3000248`)
    ///
    /// * The caller is responsible for reading the response
    /// * The command SHOULD NOT include the CRLF terminator
    pub fn send_bytes(&mut self, command: impl AsRef<[u8]>) -> Result<usize> {
        let writer = self.stream.get_mut();
        // Write the command and terminal char
        let bytes = writer.write(command.as_ref())? + writer.write(b"\r\n")?;
        // Flush the buffer
        writer.flush()?;
        Ok(bytes)
    }

    /// Read any data from the stream into a RawResponse
    ///
    /// This function attempts to automatically determine if the response is muliti-line based
    /// on the response code.
    ///
    /// Note that this *will not* work for response codes that are not supported by [`Kind`].
    /// If you are using extensions/commands not implemented by Brokaw, please use
    /// [`NntpConnection::read_response`] to configure multiline support manually.
    pub fn read_response_auto(&mut self) -> Result<RawResponse> {
        self.read_response(None)
    }

    /// Read an NNTP response from the connection
    ///
    /// # Multiline Responses
    ///
    /// If `is_multiline` is set to None then the connection use [`ResponseCode::is_multiline`]
    /// to determine if it should expect a multiline response.
    /// This behavior can be overridden by manually specifying `Some(true)` or `Some(false)`
    pub fn read_response(&mut self, is_multiline: Option<bool>) -> Result<RawResponse> {
        self.first_line_buf.truncate(0);
        self.data_blocks_buf.truncate(0);
        let resp_code = read_initial_response(&mut self.stream, &mut self.first_line_buf)?;

        let data_blocks = match (is_multiline, resp_code.is_multiline()) {
            // Check for data blocks if the caller tells us to OR the kind is multiline
            (Some(true), _) | (_, true) => {
                trace!("Parsing data blocks for response {}", u16::from(resp_code));

                // FIXME(ops): Consider pre-allocating this buffer
                let mut line_boundaries = Vec::with_capacity(10);

                let mut stream = match self.config.compression {
                    Some(c) if c.use_decoder(&self.first_line_buf) => {
                        trace!("Compression enabled, wrapping stream with decoder");
                        c.decoder(&mut self.stream)
                    }
                    _ => {
                        trace!("Using passthrough decoder");
                        Decoder::Passthrough(&mut self.stream)
                    }
                };

                read_data_blocks(&mut stream, &mut self.data_blocks_buf, &mut line_boundaries)?;

                Some(DataBlocks {
                    payload: self.data_blocks_buf.clone(),
                    line_boundaries,
                })
            }
            (Some(false), _) => None, // The caller says not to look for data blocks
            _ => None,
        };

        let resp = RawResponse {
            code: resp_code,
            first_line: self.first_line_buf.clone(),
            data_blocks,
        };

        self.reset_buffers();

        Ok(resp)
    }

    /// Reset the connection's buffers to their initial size
    ///
    /// This should be run after reading responses to prevent the buffers from growing unbounded
    fn reset_buffers(&mut self) {
        // Honestly we should probably just use the bytes create, it seems better suited to
        //  what we want in this layer
        self.first_line_buf
            .truncate(self.config.first_line_buf_size);
        self.first_line_buf.shrink_to_fit();

        self.data_blocks_buf
            .truncate(self.config.data_blocks_buf_size);
        self.data_blocks_buf.shrink_to_fit();
    }

    /// Get a ref to the underlying NntpStream
    pub fn stream(&self) -> &io::BufReader<NntpStream> {
        &self.stream
    }

    /// Get a mutable ref to the underlying NntpStream
    ///
    /// This can be useful if you want to handle response parsing and/or control buffering
    pub fn stream_mut(&mut self) -> &mut io::BufReader<NntpStream> {
        &mut self.stream
    }

    /// Get the configuration of the connection
    pub fn config(&self) -> &ConnectionConfig {
        &self.config
    }
}

/// A buffered NntpStream
pub type BufNntpStream = io::BufReader<NntpStream>;

/// A builder for [`NntpConnection`]
#[derive(Clone, Debug)]
pub struct ConnectionConfig {
    pub(crate) compression: Option<Compression>,
    pub(crate) tls_config: Option<TlsConfig>,
    pub(crate) read_timeout: Option<Duration>,
    pub(crate) write_timeout: Option<Duration>,
    pub(crate) first_line_buf_size: usize,
    pub(crate) data_blocks_buf_size: usize,
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        ConnectionConfig {
            compression: None,
            tls_config: None,
            read_timeout: None,
            write_timeout: None,
            first_line_buf_size: 128,
            data_blocks_buf_size: 16 * 1024,
        }
    }
}

impl ConnectionConfig {
    /// Create a new connection builder
    pub fn new() -> ConnectionConfig {
        Default::default()
    }

    /// Set the compression type on the connection
    pub fn compression(&mut self, compression: Option<Compression>) -> &mut Self {
        self.compression = compression;
        self
    }

    /// Configure TLS on the connection
    pub fn tls_config(&mut self, config: Option<TlsConfig>) -> &mut Self {
        self.tls_config = config;
        self
    }

    /// Use the default TLS implementation
    pub fn default_tls(&mut self, domain: impl AsRef<str>) -> Result<&mut Self> {
        let domain = domain.as_ref().to_string();
        let tls_config = TlsConfig::default_connector(domain)?;
        self.tls_config = Some(tls_config);

        Ok(self)
    }

    /// Set the read timeout on the socket
    pub fn read_timeout(&mut self, dur: Option<Duration>) -> &mut Self {
        self.read_timeout = dur;
        self
    }

    /// Set the size of the buffer used to read the first line
    pub fn first_line_buf_size(&mut self, s: usize) -> &mut Self {
        self.first_line_buf_size = s;
        self
    }

    /// Set the size of the buffer used to read data blocks
    pub fn data_blocks_buf_size(&mut self, s: usize) -> &mut Self {
        self.data_blocks_buf_size = s;
        self
    }

    /// Create a connection from the config
    pub fn connect(&self, addr: impl ToSocketAddrs) -> Result<(NntpConnection, RawResponse)> {
        NntpConnection::connect(addr, self.clone())
    }
}

/// Read the initial response from a stream
///
/// Per [RFC 3977](https://tools.ietf.org/html/rfc3977#section-3.1) the initial response
/// should not exceed 512 bytes
fn read_initial_response<S: io::BufRead>(
    stream: &mut S,
    buffer: &mut Vec<u8>,
) -> Result<ResponseCode> {
    stream.read_until(b'\n', buffer)?;
    let (_initial_line_buffer, resp) = parse_first_line(&buffer).map_err(|_e| {
        io::Error::new(
            ErrorKind::InvalidData,
            "Failed to parse first line of response",
        )
    })?;

    // This made it past the parser -> infallible
    let code_str = std::str::from_utf8(resp.code).unwrap();
    // All three digit integers will fit w/in u16 -> also infallible
    let code_u16 = u16::from_str(code_str).unwrap();

    Ok(code_u16.into())
}

/// Read multi-line data block portion from a stream
///
/// * The data will be read line-by-line into the provided `buffer`
/// * The `line_boundaries` vector will contain a list two-tuples containing the start and ending
///   of every line within the `buffer`
/// * Note that depending on the command the total data size may be on the order of several megabytes!
fn read_data_blocks<S: io::BufRead>(
    stream: &mut S,
    buffer: &mut Vec<u8>,
    line_boundaries: &mut Vec<(usize, usize)>,
) -> Result<()> {
    let mut read_head = 0;
    trace!("Reading data blocks...");

    // n.b. - icky imperative style so that we have zero allocations outside of the reader
    loop {
        // n.b. - read_until will _append_ data from the current end of the vector
        let bytes_read = stream.read_until(b'\n', buffer)?;

        let (_empty, line) = parse_data_block_line(&buffer[read_head..]).map_err(|e| {
            trace!("parse_data_block_line failed -- {:?}", e);
            io::Error::new(
                ErrorKind::InvalidData,
                format!(
                    "Failed to parse line {} of data blocks",
                    line_boundaries.len() + 1
                ),
            )
        })?;
        // we keep track of line boundaries rather than slices as borrowck won't allow
        // us to reuse the buffer AND keep track of sub-slices within it
        line_boundaries.push((read_head, read_head + bytes_read));

        // n.b. we use bytes read rather than the length of line so that we don't drop the
        // terminators
        read_head += bytes_read;

        if is_end_of_datablock(line) {
            trace!(
                "Read {} bytes of data across {} lines",
                read_head,
                line_boundaries.len()
            );
            break;
        }
    }

    Ok(())
}
