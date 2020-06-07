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

/// Typed NNTP Responses for individual commands
pub mod response;

/// NNTP Response codes
pub mod response_code;

pub mod prelude {
    pub use super::response_code::*;

    pub use super::response::*;

    pub use super::command::NntpCommand;
}
