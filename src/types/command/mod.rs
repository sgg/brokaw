use std::fmt;

/// An NNTP command
///
/// All `NntpCommands` must implement [`fmt::Display`] such that
/// [`fmt`](fmt::Display::fmt) returns the string that should be sent over the wire.
///
/// # Example: Implementing LISTGROUP
/// ```
/// use std::fmt;
/// use brokaw::types::command::NntpCommand;
///
/// #[derive(Clone, Debug)]
/// pub struct ListGroup {
///     group: Option<String>,
///     range: Option<(u32, u32)>,
/// }
///
/// impl fmt::Display for ListGroup {
///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
///         write!(f, "LISTGROUP")?;
///
///         if let Some(group) = &self.group {
///             write!(f, " {}", &group)?;
///         }
///
///         if let Some((low, high)) = &self.range {
///             write!(f, " {}-{}", low, high)?;
///         }
///         Ok(())
///     }
/// }
///
/// impl NntpCommand for ListGroup {}
///
/// let cmd = ListGroup {
///     group: Some("misc.test".to_string()),
/// range: Some((10, 20))
/// };
///
/// assert_eq!(cmd.to_string(), "LISTGROUP misc.test 10-20")
/// ```
pub trait NntpCommand: fmt::Display {}

pub type ArticleNumber = u32;

/// Commands specified in [RFC 3977](https://tools.ietf.org/html/rfc3977#appendix-B)
mod rfc3977;

pub use rfc3977::*;

/// Commands specified in [RFC 2980](https://tools.ietf.org/html/rfc2980)
mod rfc2980;

pub use rfc2980::*;

/// AUTHINFO commands specified in [RFC 4643](https://tools.ietf.org/html/rfc4643)
mod rfc4643;

pub use rfc4643::*;
