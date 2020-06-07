use std::fmt;
use std::io;
use std::io::{ErrorKind, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::str::FromStr;
use std::time::Duration;

use log::*;
use native_tls::TlsConnector;

use crate::raw::error::Result;
use crate::raw::parse::{is_end_of_datablock, parse_data_block_line, parse_first_line};
use crate::raw::response::{DataBlocks, RawResponse};
use crate::raw::stream::NntpStream;
use crate::types::command::NntpCommand;
use crate::types::prelude::*;

// FIXME: Derive Debug once TlsConnector implements Debug
/// TLS configuration for an NNTP Client/Connection
#[derive(Clone)]
pub struct TlsConfig {
    connector: TlsConnector,
    domain: String,
}

impl TlsConfig {
    /// Create a `TlsConfig` for the
    ///
    /// The `domain` will be passed to [`TlsConnector::connect`] for certificate validatoin
    /// when an `NntpConnection` is created
    pub fn new(domain: String, connector: TlsConnector) -> Self {
        Self { connector, domain }
    }

    /// Create a `TlsConfig` with the system default TLS settings
    pub fn default_connector(domain: impl AsRef<str>) -> Result<Self> {
        let connector = TlsConnector::new()?;
        Ok(Self {
            connector,
            domain: domain.as_ref().to_string(),
        })
    }

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

/// A connection to an NNTP Server
///
/// Please note that NNTP is a STATEFUL protocol and the Connection DOES NOT maintain any information
/// about this state. For a more ergonomic client please see [`NntpClient`](crate::client::NntpClient).
///
/// The [`NntpConnection::command`] method can be used perform basic command/receive operations
///
/// For finer grained control over I/O please see the following:
/// * [`send`](Self::send) & [`send_bytes`](Self::send_bytes) for writing
/// * [`read_response`](Self::read_response) for reading known response types
/// * [`read_initial_response`] & [`read_data_blocks`] for reading when you wish to manage buffers manually
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
///         .iter()
///         .map(|line| std::str::from_utf8(line).unwrap())
///         .any(|l| l.contains("VERSION"));
///
///     assert!(contains_version);
///     Ok(())
/// }
/// ```
#[derive(Debug)]
pub struct NntpConnection {
    stream: io::BufReader<NntpStream>,
    tls_config: Option<TlsConfig>,
    first_line_buffer: Vec<u8>,
}

impl NntpConnection {
    /// Connect to an NNTP server
    pub fn connect<A: ToSocketAddrs>(
        addr: A,
        tls_config: Option<TlsConfig>,
        read_timeout: Option<Duration>,
    ) -> Result<(Self, RawResponse)> {
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

        let mut conn = Self {
            stream: io::BufReader::new(nntp_stream),
            tls_config,
            first_line_buffer: Vec::with_capacity(128),
        };

        let initial_resp = conn.read_response()?;

        Ok((conn, initial_resp))
    }

    /// Send a command to the server and read the response
    ///
    /// This function will:
    /// 1. Block while reading the response
    /// 2. Parse the response
    /// 2. This function *may* allocate depending ont he nature of the response
    pub fn command<C: NntpCommand>(&mut self, command: &C) -> Result<RawResponse> {
        self.send(command)?;
        let resp = self.read_response()?;
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
    /// Note that if the response code is not supported then it is the caller's responsibility
    /// to determine whether or not to keep reading data.
    pub fn read_response(&mut self) -> Result<RawResponse> {
        self.first_line_buffer.truncate(0);
        let resp_code = read_initial_response(&mut self.stream, &mut self.first_line_buffer)?;

        let data_blocks = match resp_code {
            ResponseCode::Known(kind) if kind.is_multiline() => {
                trace!(
                    "Response {} indicates a multiline response, parsing data blocks",
                    kind as u16
                );

                // FIXME(perf): Consider preallocating these buffers and allowing users to tune
                //     them upon connection creation
                let mut datablocks = Vec::with_capacity(1024);
                let mut line_boundaries = Vec::with_capacity(10);
                read_data_blocks(&mut self.stream, &mut datablocks, &mut line_boundaries)?;
                Some(DataBlocks {
                    payload: datablocks,
                    line_boundaries,
                })
            }
            ResponseCode::Known(kind) => {
                trace!(
                    "Response {} is not multi-line, skipping data block parsing",
                    kind as u16
                );
                None
            }
            ResponseCode::Unknown(_) => None,
        };

        Ok(RawResponse {
            code: resp_code,
            first_line: self.first_line_buffer.clone(),
            data_blocks,
        })
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

    /// Get the TLS configuration of the connection
    pub fn tls_config(&self) -> Option<&TlsConfig> {
        self.tls_config.as_ref()
    }
}

/// Read the initial response from a stream
///
/// Per [RFC 3977](https://tools.ietf.org/html/rfc3977#section-3.1) the initial response
/// should not exceed 512 bytes
pub fn read_initial_response<S: io::BufRead>(
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
pub fn read_data_blocks<S: io::BufRead>(
    stream: &mut S,
    buffer: &mut Vec<u8>,
    line_boundaries: &mut Vec<(usize, usize)>,
) -> Result<()> {
    let mut read_head = 0;
    trace!("Reading multi-line datablocks...");

    // n.b. - icky imperative style so that we have zero allocations outside of the reader
    loop {
        // n.b. - read_until will _append_ data from the current end of the vector
        let bytes_read = stream.read_until(b'\n', buffer)?;

        let (_empty, line) = parse_data_block_line(&buffer[read_head..])
            .map_err(|_e| io::Error::new(ErrorKind::InvalidData, ""))?;
        // we keep track of line boundaries rather than slices as borrowck won't allow
        // us to reuse the buffer AND keep track of sub-slices within it
        line_boundaries.push((read_head, read_head + bytes_read));

        // n.b. we use bytes read rather than the length of line so that we don't drop the
        // terminators
        read_head += bytes_read;

        if is_end_of_datablock(line) {
            trace!(
                "Read {} bytes of data w/ boundaries {:?}",
                read_head,
                line_boundaries
            );
            break;
        }
    }

    Ok(())
}
