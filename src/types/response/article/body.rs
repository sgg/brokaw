use std::convert::TryFrom;

use crate::error::{Error, Result};
use crate::types::prelude::*;
use crate::types::response::article::iter::*;
use crate::types::response::util::{err_if_not_kind, process_article_first_line};

/// An article body returned by the [`BODY`](https://tools.ietf.org/html/rfc3977#section-6.2.3)
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Body {
    /// The number of the article unique to a particular newsgroup
    pub number: ArticleNumber,
    /// The unique message id for the article
    pub message_id: String,
    pub(crate) payload: Vec<u8>,
    pub(crate) line_boundaries: Vec<(usize, usize)>,
}

impl Body {
    /// The number of the article relative to the group it was retrieved from
    pub fn number(&self) -> ArticleNumber {
        self.number
    }

    /// The message id of the article
    pub fn message_id(&self) -> &str {
        &self.message_id
    }

    /// The raw contents of the body
    pub fn body(&self) -> &[u8] {
        &self.payload
    }

    /// An iterator over the lines in the body of the article
    pub fn lines(&self) -> Lines<'_> {
        Lines {
            payload: &self.payload,
            inner: self.line_boundaries.iter(),
        }
    }

    /// An iterator over the lines of the body without the CRLF terminators
    pub fn unterminated(&self) -> Unterminated<'_> {
        Unterminated {
            inner: self.lines(),
        }
    }
}

impl TryFrom<&RawResponse> for Body {
    type Error = Error;

    fn try_from(resp: &RawResponse) -> Result<Self> {
        err_if_not_kind(resp, Kind::Body)?;

        let (number, message_id) = process_article_first_line(&resp)?;

        let DataBlocks {
            payload,
            line_boundaries,
        } = resp
            .data_blocks
            .as_ref()
            .ok_or_else(Error::missing_data_blocks)?
            .clone();

        Ok(Self {
            number,
            message_id,
            payload,
            line_boundaries,
        })
    }
}
