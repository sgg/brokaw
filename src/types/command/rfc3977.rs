use std::fmt;

use crate::types::prelude::{ArticleNumber, NntpCommand};

/// Retrieve an article's header and body
#[derive(Clone, Debug)]
pub enum Article {
    /// Globally unique message ID
    MessageId(String),
    /// Article number relative to the current group
    Number(ArticleNumber),
    /// Currently selected article
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

/// Retrieve the body for an Article
#[derive(Clone, Debug)]
pub enum Body {
    /// Globally unique message ID
    MessageId(String),
    /// Article number relative to the current group
    Number(ArticleNumber),
    /// Currently selected article
    Current,
}

impl fmt::Display for Body {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Body::MessageId(id) => write!(f, "BODY {}", id),
            Body::Number(num) => write!(f, "BODY {}", num),
            Body::Current => write!(f, "BODY"),
        }
    }
}

impl NntpCommand for Body {}

/// Get the capabilities provided by the server
#[derive(Clone, Copy, Debug)]
pub struct Capabilities;

impl fmt::Display for Capabilities {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CAPABILITIES")
    }
}

impl NntpCommand for Capabilities {}

/// Get the server time
#[derive(Clone, Copy, Debug)]
struct Date;

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DATE")
    }
}

impl NntpCommand for Date {}

/// Select a group
#[derive(Clone, Debug)]
pub struct Group(pub String);

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GROUP {}", self.0)
    }
}

impl NntpCommand for Group {}

/// Retrieve a specific header from one or more articles
#[derive(Clone, Debug)]
pub enum Hdr {
    /// A single article by message ID
    MessageId {
        /// The name of the header
        field: String,
        /// The unique message id of the article
        id: String,
    },
    /// A range of articles
    Range {
        /// The name of the header
        field: String,
        /// The low number of the article range
        low: ArticleNumber,
        /// The high number of the article range
        high: ArticleNumber,
    },
    /// The current article
    Current {
        /// The name of the header
        field: String,
    },
}

impl fmt::Display for Hdr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Hdr::MessageId { field, id } => write!(f, "HDR {} {}", field, id),
            Hdr::Range { field, low, high } => write!(f, "HDR {} {}-{}", field, low, high),
            Hdr::Current { field } => write!(f, "HDR {}", field),
        }
    }
}

impl NntpCommand for Hdr {}

/// Retrieve the headers for an article
#[derive(Clone, Debug)]
pub enum Head {
    /// Globally unique message ID
    MessageId(String),
    /// Article number relative to the current group
    Number(ArticleNumber),
    /// Currently selected article
    Current,
}

impl fmt::Display for Head {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Head::MessageId(id) => write!(f, "HEAD {}", id),
            Head::Number(num) => write!(f, "HEAD {}", num),
            Head::Current => write!(f, "HEAD"),
        }
    }
}

impl NntpCommand for Head {}

/// Retrieve help text about the servers capabilities
struct Help;

impl fmt::Display for Help {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HELP")
    }
}

impl NntpCommand for Help {}

/// Inform the server that you have an article for upload
struct IHave(String);

impl fmt::Display for IHave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IHAVE {}", self.0)
    }
}

impl NntpCommand for IHave {}

/// Attempt to set the current article to the previous article number
struct Last;

impl fmt::Display for Last {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LAST")
    }
}

impl NntpCommand for Last {}

/// Retrieve a list of information from the server
///
/// Not all LIST keywords are supported by all servers.
///
/// ## Usage Note
///
/// If you want to send LIST without any keywords simply send [`List::Active`] as they are equivalent.
#[derive(Clone, Debug)]
#[allow(missing_docs)]
pub enum List {
    /// Return a list of active newsgroups
    ///
    /// [RFC 3977 7.6.3](https://tools.ietf.org/html/rfc3977#section-7.6.3)
    Active { wildmat: Option<String> },
    /// Return information about when news groups were created
    ///
    /// [RFC 3977 7.6.4](https://tools.ietf.org/html/rfc3977#section-7.6.4)
    ActiveTimes { wildmat: Option<String> },
    /// List descriptions of newsgroups available on the server
    ///
    /// [RFC 3977 7.6.6](https://tools.ietf.org/html/rfc3977#section-7.6.6)
    Newsgroups { wildmat: Option<String> },
    /// Retrieve information about the Distribution header for news articles
    ///
    /// [RFC 3977 7.6.5](https://tools.ietf.org/html/rfc3977#section-7.6.5)
    DistribPats,
    /// Return field descriptors for headers returned by OVER/XOVER
    ///
    /// [RFC 3977 8.4](https://tools.ietf.org/html/rfc3977#section-8.4)
    OverviewFmt,
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn print_wildmat(f: &mut fmt::Formatter<'_>, wildmat: Option<&String>) -> fmt::Result {
            if let Some(w) = wildmat.as_ref() {
                write!(f, " {}", w)
            } else {
                Ok(())
            }
        }

        write!(f, "LIST")?;
        match self {
            List::Active { wildmat } => {
                write!(f, " ACTIVE")?;
                print_wildmat(f, wildmat.as_ref())
            }
            List::OverviewFmt => write!(f, " OVERVIEW.FMT"),
            List::ActiveTimes { wildmat } => {
                write!(f, " ACTIVE TIMES")?;
                print_wildmat(f, wildmat.as_ref())
            }
            List::Newsgroups { wildmat } => {
                write!(f, " ACTIVE TIMES")?;
                print_wildmat(f, wildmat.as_ref())
            }
            List::DistribPats => write!(f, " DISTRIB.PATS"),
        }
    }
}

impl NntpCommand for List {}

/// Enable reader mode on a mode switching server
#[derive(Clone, Copy, Debug)]
pub struct ModeReader;

impl fmt::Display for ModeReader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MODE READER")
    }
}

impl NntpCommand for ModeReader {}

// implement NEWGROUPS

/// List newsgroups created since a specific date
#[derive(Clone, Debug)]
pub struct NewGroups {
    /// The date from which to list newsgroups
    pub date: String,
    /// the time from which to list newsgroups
    pub time: String,
    /// Optional distributions to include
    pub distributions: Option<String>,
}

impl fmt::Display for NewGroups {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "NEWGROUPS {} {} {}",
            self.date,
            self.time,
            self.distributions.as_deref().unwrap_or("")
        )
    }
}

impl NntpCommand for NewGroups {}

// TODO(commands) implement NEWNEWS

/// Attempt to set the current article to the next article number
#[derive(Clone, Copy, Debug)]
pub struct Next;

impl fmt::Display for Next {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NEXT")
    }
}

impl NntpCommand for Next {}

/// Retrieve all of the fields (e.g. headers/metadata) for one or more articles
#[derive(Clone, Debug)]
pub enum Over {
    /// A single article by message ID
    MessageId(String),
    /// A range of articles
    Range {
        /// The low number of the article
        low: ArticleNumber,
        /// The high number of the article
        high: ArticleNumber,
    },
    /// The current article
    Current,
}

impl fmt::Display for Over {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Over::MessageId(id) => write!(f, "OVER {}", id),
            Over::Range { low, high } => write!(f, "OVER {}-{}", low, high),
            Over::Current => write!(f, "OVER"),
        }
    }
}

impl NntpCommand for Over {}

// TODO(commands) complete POST implementation
/*
/// Post an article to the news server
///
/// POSTING is a two part exchange. The [`Initial`](Post::Initial) variant is used
/// to determine if the server will accept the article while the [`Article`](Post::Article) is
/// used to send the data.
///
///
/// For more information see [RFC 3977 6.3.1](https://tools.ietf.org/html/rfc3977#section-6.3.1)
#[derive(Clone, Debug)]
enum Post<'a> {
    Initial,
    /// The article body NOT including the terminating sequence
    Article(&'a [u8])
}

impl NntpCommand for Post<'_> {}

impl fmt::Display for Post<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Post::Initial => write!(f, "POST"),
            Post::Article(body) => {
                unimplemented!()
            },
        }
    }
}
*/

/// Close the connection
#[derive(Clone, Copy, Debug)]
pub struct Quit;

impl fmt::Display for Quit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "QUIT")
    }
}

impl NntpCommand for Quit {}

/// Check if an article exists in the newsgroup
#[derive(Clone, Debug)]
pub enum Stat {
    /// Globally unique message ID
    MessageId(String),
    /// Article number relative to the current group
    Number(ArticleNumber),
    /// Currently selected article
    Current,
}

impl fmt::Display for Stat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stat::MessageId(id) => write!(f, "STAT {}", id),
            Stat::Number(num) => write!(f, "STAT {}", num),
            Stat::Current => write!(f, "STAT"),
        }
    }
}

impl NntpCommand for Stat {}
