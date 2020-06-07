use crate::raw::response::RawResponse;
use crate::types::response_code::ResponseCode;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// TODO
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
    Deserialization { msg: String }, // FIXME: remove Deserialization
    #[error("{0}")]
    Deserialization2(String),
}

impl Error {
    pub(crate) fn bad_response(resp: RawResponse) -> Self {
        Error::BadResponse {
            code: resp.code(),
            resp,
            msg: None,
        }
    }
    pub(crate) fn de(msg: impl AsRef<str>) -> Self {
        Error::Deserialization2(msg.as_ref().to_string())
    }

    // FIXME: missing_field and parse_error are redundent
    pub(crate) fn missing_field(name: impl AsRef<str>) -> Self {
        Error::Deserialization2(format!("Missing field `{}`", name.as_ref()))
    }

    pub(crate) fn parse_error(name: impl AsRef<str>) -> Self {
        Error::Deserialization2(format!("Could not parse field `{}`", name.as_ref()))
    }

    pub(crate) fn missing_data_blocks() -> Self {
        Error::Deserialization2("Response is missing multi-line data blocks".to_string())
    }

    pub(crate) fn invalid_data_blocks(msg: impl AsRef<str>) -> Self {
        Error::Deserialization2(format!("Invalid data-block section -- {}", msg.as_ref()))
    }
}

pub type Result<T> = std::result::Result<T, Error>;
