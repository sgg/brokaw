use std::convert::TryFrom;
use std::fmt;
use std::result::Result as StdResult;
use std::str::from_utf8;

use log::*;

use crate::error::{Error, Result};
use crate::types::prelude::*;
use crate::types::response::article::iter::{Lines, Unterminated};
use crate::types::response::article::parse::take_headers;
use crate::types::response::util::{err_if_not_kind, process_article_first_line};

/// A binary Netnews article
///
/// A `BinaryArticle` is created by calling `try_from` with a [`RawResponse`].
///
/// For text articles, consider converting this article into a [`TextArticle`].
///
/// # Implementation Notes
///
/// Articles aim to follow [RFC 3977](https://tools.ietf.org/html/rfc3977#section-3.6)
/// as closely as possible while remaining ergonomic.
///
/// 1. Response parsing will fail if the header names are not UTF-8
/// 2. The header contents will be lossily converted to UTF-8
/// 3. There are no formatting constraints on the body
///
/// TODO Example
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BinaryArticle {
    pub(crate) number: ArticleNumber,
    pub(crate) message_id: String,
    pub(crate) headers: Headers,
    pub(crate) body: Vec<u8>,
    pub(crate) line_boundaries: Vec<(usize, usize)>,
}

impl BinaryArticle {
    /// The number of the article relative to the group it was retrieved from
    pub fn number(&self) -> ArticleNumber {
        self.number
    }

    /// The message id of the article
    pub fn message_id(&self) -> &str {
        &self.message_id
    }

    /// The headers on the article
    pub fn headers(&self) -> &Headers {
        &self.headers
    }

    /// The raw contents of the body
    pub fn body(&self) -> &[u8] {
        &self.body
    }

    /// The number of lines in the body
    pub fn lines_len(&self) -> usize {
        self.line_boundaries.len()
    }

    /// An iterator over the lines in the body of the article
    pub fn lines(&self) -> Lines<'_> {
        Lines {
            payload: &self.body,
            inner: self.line_boundaries.iter(),
        }
    }

    /// An iterator over the lines of the body without the CRLF terminators
    pub fn unterminated(&self) -> Unterminated<'_> {
        Unterminated {
            inner: self.lines(),
        }
    }

    /// Convert the article into a [`TextArticle`]
    ///
    /// This will return an error if the body is not valid UTF-8
    pub fn to_text(&self) -> Result<TextArticle> {
        let headers = self.headers.clone();

        let body: Vec<String> = self
            .unterminated()
            .map(|l| from_utf8(l).map(ToString::to_string))
            .collect::<StdResult<_, _>>()?;

        let number = self.number;
        let message_id = self.message_id.clone();
        Ok(TextArticle {
            number,
            message_id,
            headers,
            body,
        })
    }

    /// Convert the article into a [`TextArticle`] including invalid characters.
    ///
    /// This function is analogous to calling is [`String::from_utf8_lossy`] on every line in the body
    pub fn to_text_lossy(&self) -> TextArticle {
        let headers = self.headers.clone();

        let body = self
            .unterminated()
            .map(String::from_utf8_lossy)
            .map(|cow| cow.to_string())
            .collect::<Vec<String>>();

        let number = self.number;
        let message_id = self.message_id.clone();
        TextArticle {
            number,
            message_id,
            headers,
            body,
        }
    }
}

impl fmt::Display for BinaryArticle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let num_headers = self.headers.len();
        let body_size = self.body.len();
        let lines = self.lines_len();

        write!(
            f,
            "BinaryArticle({} headers, {}B body, {} lines)",
            num_headers, body_size, lines
        )
    }
}

impl TryFrom<&RawResponse> for BinaryArticle {
    type Error = Error;
    /// Convert a raw response into an article
    ///
    /// For the specification see RFC 3977 sections:
    ///
    /// * [response-220-content](https://tools.ietf.org/html/rfc3977#section-9.4.2)
    /// * [article](https://tools.ietf.org/html/rfc3977#section-9.7)
    fn try_from(resp: &RawResponse) -> Result<Self> {
        err_if_not_kind(resp, Kind::Article)?;
        let (number, message_id) = process_article_first_line(&resp)?;

        let data_blocks = resp
            .data_blocks
            .as_ref()
            .ok_or_else(Error::missing_data_blocks)?;

        let (body, headers) = take_headers(&data_blocks.payload()).map_err(|e| match e {
            nom::Err::Incomplete(n) => Error::Deserialization(format!("{:?}", n)),
            nom::Err::Error((_, kind)) | nom::Err::Failure((_, kind)) => {
                Error::invalid_data_blocks(format!("{:?}", kind))
            }
        })?;

        let bytes_read = data_blocks.payload.len() - body.len();
        trace!("Read {} bytes as headers", bytes_read);

        let line_boundaries = data_blocks
            .line_boundaries
            .iter()
            .skip_while(|(start, _end)| start < &bytes_read)
            .map(|(start, end)| (start - bytes_read, end - bytes_read))
            .collect::<Vec<_>>();

        Ok(Self {
            number,
            message_id,
            headers,
            body: body.to_vec(),
            line_boundaries,
        })
    }
}
