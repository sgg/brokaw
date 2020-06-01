use std::io;
use crate::raw::response::RawResponse;
use crate::types::response_code::ResponseCode;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    ///
    ///
    /// For example, asking for a non-existent group
    #[error("Server returned {code:?}")]
    BadResponse {
        code: ResponseCode,
        resp: RawResponse,
        msg: Option<String>,
    },
    #[error(transparent)]
    /// Error with the underlying connection
    Connection(#[from] crate::raw::error::Error),
    #[error("Config Error -- {0}")]
    Config(String),
    #[error("{msg}")]
    Deserialization {
        msg: String
    },
    #[error("{0}")]
    Deserialization2(String)
}

impl Error {
    pub(crate) fn bad_response(resp: RawResponse) -> Self {
        Error::BadResponse {
            code: resp.code(),
            resp,
            msg: None
        }
    }
    pub(crate) fn de(msg: impl AsRef<str>) -> Self {
        Error::Deserialization2(msg.as_ref().to_string())
    }

    pub(crate) fn missing_field(name: impl AsRef<str>) -> Self {
        Error::Deserialization2(format!("Missing field `{}`", name.as_ref()))
    }

    pub(crate) fn parse_error(name: impl AsRef<str>) -> Self {
        Error::Deserialization2(format!("Could not parse field `{}`", name.as_ref()))
    }
}

pub type Result<T> = std::result::Result<T, Error>;
