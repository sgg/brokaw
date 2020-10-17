/// A data-structure that represents an NNTP command
///
/// All `NntpCommands` must implement [`Encode`] such that
/// [`encode`](Encode::encode) returns the bytes that should be sent over the wire.
///
/// If the command can be represented using UTF-8, one can simply implement [`ToString`]
/// (directly or via [`fmt::Display`](std::fmt::Display) as [`Encode`] is automatically implemented for
/// types that implement [`ToString`].
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
pub trait NntpCommand: Encode {}

/// A type that can be serialized for transmission
///
/// A blanket implementation is provided for types implementing [`ToString`].
pub trait Encode {
    /// Return a vector of bytes that can be sent to an NNTP server
    fn encode(&self) -> Vec<u8>;
}

impl<T: ToString> Encode for T {
    fn encode(&self) -> Vec<u8> {
        self.to_string().into()
    }
}

/// Commands specified in [RFC 3977](https://tools.ietf.org/html/rfc3977#appendix-B)
mod rfc3977;

#[doc(inline)]
pub use rfc3977::*;

/// Commands specified in [RFC 2980](https://tools.ietf.org/html/rfc2980)
mod rfc2980;

#[doc(inline)]
pub use rfc2980::*;

/// AUTHINFO commands specified in [RFC 4643](https://tools.ietf.org/html/rfc4643)
mod rfc4643;

#[doc(inline)]
pub use rfc4643::*;

mod xfeature;

#[doc(inline)]
pub use xfeature::*;
