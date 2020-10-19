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

// TODO(commands) implement NEWGROUPS

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

pub use post::Post;

/// Post commands and builders
pub mod post {
    use std::collections::{HashMap, HashSet};

    use crate::types::command::Encode;
    use crate::types::prelude::*;

    /// A line terminator for a multi-line data block
    const CRLF: &[u8] = b"\r\n";

    /// Post an article to a newsgroup
    ///
    /// This command should be constructed using [`Post::builder`].
    #[derive(Clone, Debug)]
    pub struct Post {
        headers: HashMap<String, HashSet<String>>,
        body: Vec<u8>,
    }

    impl Post {
        /// Create a builder for the post
        pub fn builder() -> Builder {
            Builder::new()
        }
    }

    /// A builder for [`Post`]
    #[derive(Clone, Debug, Default)]
    pub struct Builder {
        headers: HashMap<String, HashSet<String>>,
        body: Vec<u8>,
    }

    impl Builder {
        /// Create a new Post builder
        pub fn new() -> Builder {
            Self::default()
        }

        /// Add a header
        ///
        /// Note that per [RFC 5322](https://tools.ietf.org/html/rfc5322#section-3.6) a header
        /// may be duplicated. As such duplicates will be maintained.
        ///
        /// If you wish to overwrite a header that already exists see [`set_header`].
        pub fn header(&mut self, name: impl AsRef<str>, value: impl AsRef<str>) -> &mut Self {
            let name = name.as_ref().to_string();
            let value = value.as_ref().to_string();

            self.headers.entry(name).or_default().insert(value);

            self
        }

        /// Set a header, overwriting any pre-existing values
        pub fn set_header(&mut self, name: impl AsRef<str>, value: impl AsRef<str>) -> &mut Self {
            let name = name.as_ref().to_string();
            let value = value.as_ref().to_string();

            let entry = self.headers.entry(name).or_default();
            entry.clear();
            entry.insert(value);

            self
        }

        /// Add multiple headers to the article builder
        pub fn headers<K, V>(&mut self, headers: impl IntoIterator<Item = (K, V)>) -> &mut Self
        where
            K: AsRef<str>,
            V: AsRef<str>,
        {
            headers.into_iter().for_each(|(name, value)| {
                self.header(name, value);
            });

            self
        }

        /// Set the body for the article
        pub fn body(&mut self, body: impl AsRef<[u8]>) -> &mut Self {
            self.body = body.as_ref().to_owned();
            self
        }

        /// Build the [`Post`] command
        pub fn build(&self) -> Post {
            let Builder { headers, body } = self.clone();
            Post { headers, body }
        }
    }

    impl NntpCommand for Post {}

    impl Encode for Post {
        fn encode(&self) -> Vec<u8> {
            let header_sep = b": ";
            // n.b. there are probably 1s-10s of headers and iterating is cheaper than allocating
            let headers_len = self
                .headers
                .iter()
                .flat_map(|(name, vals)| {
                    let name_len = name.len();
                    vals.iter().map(move |val| (name_len, val.len()))
                })
                // add bytes for the delimiter chars
                .fold(0, |acc, (name_len, val_len)| {
                    acc + name_len + header_sep.len() + val_len + CRLF.len()
                });

            let headers_iter = self
                .headers
                .iter()
                .flat_map(|(name, vals)| vals.iter().map(move |val| (name, val)));

            let first_line = b"POST";

            // create the output buffer
            let mut buf: Vec<u8> = Vec::with_capacity(
                first_line.len() + CRLF.len() + headers_len + self.body.len() + CRLF.len() + 1,
            );

            // add the first line
            buf.extend(first_line);
            buf.extend(CRLF);
            // add the headers
            headers_iter.for_each(|(name, val)| {
                buf.extend(name.as_bytes());
                buf.extend(b": ");
                buf.extend(val.as_bytes());
                buf.extend(CRLF);
            });
            // add an empty line to mark the end of the headers
            buf.extend(CRLF);
            // add the body
            buf.extend(&self.body);
            // add the data blocks terminator
            buf.extend(CRLF);
            buf.extend(b".");

            buf
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn smoke_test() {
            let first_line = b"POST\r\n";
            let body = r#"In bug 1630935 [1], I intend to deprecate support for drawing
stretched MathML operators using the STIXGeneral fonts with a use
counter, deprecation warning, and a pref to gate the feature (off by
default on nightly)."#;

            let post = Post::builder()
                .header(
                    "Message-ID",
                    "<b976e951-174a-4aba-9cd6-628b9b3418dd@googlegroups.com>",
                )
                .header(
                    "Subject",
                    "Intent to deprecate: stretching MathML operators with STIXGeneral fonts",
                )
                .header("Newsgroups", "test.jokes")
                .header("From", "dazabani@igalia.com")
                .header("User-Agent", "G2/1.0")
                .body(body)
                .build();

            let encoded = post.encode();

            assert!(encoded.starts_with(first_line));
            assert!(encoded.ends_with(b"\r\n."));
            let trailer = format!("{}\r\n.", body);
            assert!(encoded.ends_with(trailer.as_bytes()));
        }
    }
}

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
