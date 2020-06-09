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
#[allow(missing_docs)] // FIXME(docs)
pub mod command;

/// Typed NNTP Responses for individual commands
pub mod response;

/// NNTP Response codes
#[allow(missing_docs)] // FIXME(docs)
pub mod response_code;

/// The number of an article within a newsgroup
pub type ArticleNumber = u32; // FIXME: replace alias w/ newtype

/// Re-exports of common traits and types
pub mod prelude {
    pub use crate::raw::response::{DataBlocks, RawResponse};

    pub use super::command::NntpCommand;
    pub use super::response::*;
    pub use super::response_code::*;
    pub use super::ArticleNumber;
}
