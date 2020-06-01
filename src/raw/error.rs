use std::net::TcpStream;

/// Low level API Errors
///
/// These errors represent (e.g. I/O, deserialization, parsing, etc).
/// For protocol level errors see [`crate::error::Error`]
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// The connection encountered some sort of I/O error
    #[error("IO {0}")]
    Io(#[from] std::io::Error),
    /// An error propagated from the native TLS implementation
    #[error("TLS Error -- {0}")]
    Tls(#[from] native_tls::Error),
    #[error("TLS Handshake Error -- {0}")]
    TlsHandshake(#[from] native_tls::HandshakeError<TcpStream>),
    /// The server returned data that could not be parsed
    ///
    /// This likely indicates that either the parser within Brokaw has a bug or that the server
    /// returned an out of spec response.
    /// This could also occur if an unsupported compression mechanism is enabled.
    #[error("Failed to parse response")]
    Parse,
}

/// A Result returned by the low level API
pub type Result<T> = std::result::Result<T, Error>;
