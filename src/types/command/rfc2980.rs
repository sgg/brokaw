use std::fmt;

use super::{ArticleNumber, NntpCommand};

/// Retrieve a specific header from one or more articles
#[derive(Clone, Debug)]
pub enum XHdr {
    MessageId {
        header: String,
        id: String,
    },
    Range {
        header: String,
        low: ArticleNumber,
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

pub enum XOver {
    Range {
        low: ArticleNumber,
        high: ArticleNumber,
    },
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
