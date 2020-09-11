/// Traits and types for NNTP commands
///
/// The [`NntpCommand`](command::NntpCommand) trait can be used to implement commands not (yet)
/// provided by Brokaw.
///
/// Brokaw provides implementations for most of the commands
/// in [RFC 3977](https://tools.ietf.org/html/rfc3977).
///
/// One notable exception is the [`LISTGROUP`](https://tools.ietf.org/html/rfc3977#section-6.1.2)
/// command. This command is left unimplemented as it does not adhere to the response standards
/// defined in the RFC.
pub mod command;

/// Typed NNTP responses for individual commands
pub mod response;

/// NNTP response codes
pub mod response_code;

/// The number of an article relative to a specific Newsgroup
///
/// Per [RFC 3977](https://tools.ietf.org/html/rfc3977#section-6) article numbers should fit within
/// 31-bits but has been surpassed since.
pub type ArticleNumber = u64;

/// Re-exports of traits and response types
pub mod prelude {
    pub use crate::raw::response::{DataBlocks, RawResponse};

    pub use super::command::NntpCommand;
    pub use super::response::*;
    pub use super::response_code::*;
    pub use super::ArticleNumber;
}

#[doc(inline)]
pub use command::NntpCommand;

#[doc(inline)]
pub use response::*;

#[doc(inline)]
pub use response_code::*;
