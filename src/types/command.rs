
use std::{fmt};
use nom::lib::std::fmt::Formatter;

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

/// https://tools.ietf.org/html/rfc3977#section-6.2.1
#[derive(Clone, Debug)]
pub enum Article {
    MessageId(String),
    Number(ArticleNumber),
    Current,
}

impl fmt::Display for Article {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Article::MessageId(id) => write!(f, "ARTICLE {}", id),
            Article::Number(num) => write!(f, "ARTICLE {}", num),
            Article::Current => write!(f, "ARTICLE"),
        }
    }
}

impl NntpCommand for Article {}

/// https://tools.ietf.org/html/rfc3977#section-6.1.1
#[derive(Clone, Debug)]
pub struct Group(pub String);

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GROUP {}", self.0)
    }
}

impl NntpCommand for Group {}

#[derive(Clone, Debug)]
pub enum Head {
    MessageId(String),
    Number(ArticleNumber),
}

impl fmt::Display for Head {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MessageId(id) => write!(f, "HEAD {}", id),
            Self::Number(num) => write!(f, "HEAD {}", num),
        }
    }
}

impl NntpCommand for Head {}

/// Commands for the AUTHINFO EXTENSION
///
/// https://tools.ietf.org/html/rfc4643
#[derive(Clone, Debug)]
pub enum AuthInfo {
    User(String),
    Pass(String),
}

impl fmt::Display for AuthInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthInfo::User(username) => write!(f, "AUTHINFO USER {}", username),
            AuthInfo::Pass(password) => write!(f, "AUTHINFO PASS {}", password),
        }
    }
}

impl NntpCommand for AuthInfo {}

#[derive(Clone, Copy, Debug)]
pub struct Capabilities;

impl fmt::Display for Capabilities {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CAPABILITIES")
    }
}

impl NntpCommand for Capabilities {}

pub enum List {
    OverviewFmt,
    Active,
    Counts { wildmat: Option<String> },
    NoArgs
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LIST")?;
        match self {

            List::OverviewFmt => write!(f, " OVERVIEW.FMT"),
            List::Active => write!(f, " ACTIVE"),
            List::NoArgs => Ok(()),
            List::Counts { wildmat: Some(wildmat)} => write!(f, " COUNTS {}", wildmat),
            List::Counts { wildmat: None } => write!(f, " COUNTS")
        }
    }
}


impl NntpCommand for List {}

pub enum XOver {
    Range {low: ArticleNumber, high: ArticleNumber },
    Current
}
impl fmt::Display for XOver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            XOver::Range { low, high } => write!(f, "XOVER {}-{}", low, high),
            XOver::Current => write!(f, "XOVER"),
        }
    }
}

impl NntpCommand for XOver {}

pub struct Quit;

impl fmt::Display for Quit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "QUIT")
    }
}

impl NntpCommand for Quit {}
