use std::fmt;

use crate::types::prelude::{ArticleNumber, NntpCommand};

/// Retrieve a specific header from one or more articles
#[derive(Clone, Debug)]
pub enum XHdr {
    /// A single message
    MessageId {
        /// The name of the header to retrieve
        header: String,
        /// The message ID of the article
        id: String,
    },
    /// A range of messages
    Range {
        /// The name of the header to retrieve
        header: String,
        /// The low number of the article range
        low: ArticleNumber,
        /// The high number of the article range
        high: ArticleNumber,
    },
}

impl fmt::Display for XHdr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            XHdr::MessageId { header, id } => write!(f, "XHDR {} {}", header, id),
            XHdr::Range { header, low, high } => write!(f, "XHDR {} {}-{}", header, low, high),
        }
    }
}

/// Get the headers for one or more articles
#[derive(Copy, Clone, Debug)]
pub enum XOver {
    /// A range of messages
    Range {
        /// The low number of the article range
        low: ArticleNumber,
        /// The high number of the article range
        high: ArticleNumber,
    },
    /// The current message
    Current,
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
