use std::str::Utf8Error;

use crate::types::prelude::*;

/// All of the ways that a failure can occur within Brokaw
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// This error indicates an application layer failure.
    ///
    /// For example, asking for a non-existent group will return
    /// [`NoSuchNewsGroup`](`crate::types::prelude::Kind::NoSuchNewsgroup`) (code 411),
    /// which is not a protocol error.
    #[error("Server returned {code:?} -- {msg:?}")]
    Failure {
        /// The response code
        code: ResponseCode,
        /// The raw response
        resp: RawResponse,
        /// An error message associated with the response
        msg: Option<String>,
    },
    #[error(transparent)]
    /// An error raised by the underlying connection
    ///
    /// This is usually of an I/O error or a TLS error
    Connection(#[from] crate::raw::error::Error),
    /// An error deserializing response into a concrete type
    #[error("{0}")]
    Deserialization(String),
    /// An error deserializing bytes as UTF-8
    #[error("{0}")]
    Utf8(#[from] Utf8Error),
}

impl Error {
    pub(crate) fn failure(resp: RawResponse) -> Self {
        Error::Failure {
            code: resp.code(),
            resp,
            msg: None,
        }
    }

    pub(crate) fn de(msg: impl AsRef<str>) -> Self {
        Error::Deserialization(msg.as_ref().to_string())
    }

    pub(crate) fn missing_field(name: impl AsRef<str>) -> Self {
        Error::Deserialization(format!("Missing field `{}`", name.as_ref()))
    }

    pub(crate) fn parse_error(name: impl AsRef<str>) -> Self {
        Error::Deserialization(format!("Could not parse field `{}`", name.as_ref()))
    }

    pub(crate) fn missing_data_blocks() -> Self {
        Error::Deserialization("Response is missing multi-line data blocks".to_string())
    }

    pub(crate) fn invalid_data_blocks(msg: impl AsRef<str>) -> Self {
        Error::Deserialization(format!("Invalid data-block section -- {}", msg.as_ref()))
    }
}

/// A result type returned by the library
pub type Result<T> = std::result::Result<T, Error>;
