use std::collections::{hash_map, HashMap};
use std::convert::TryFrom;

use crate::error::{Error, Result};
use crate::raw::response::RawResponse;
use crate::types::prelude::*;
use crate::types::response::article::parse::take_headers;
use crate::types::response::util::{err_if_not_kind, process_article_first_line};

/// The headers on an article
///
/// Note that per [RFC 5322](https://tools.ietf.org/html/rfc5322#section-3.6) header
/// may be repeated
///
/// [Header Folding](https://tools.ietf.org/html/rfc3977#appendix-A.1)
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Headers {
    pub(crate) inner: HashMap<String, Header>,
    pub(crate) len: u32,
}

/// An individual header
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Header {
    /// The name of the header
    pub name: String,
    /// One-or-more content values for the header
    pub content: Vec<String>,
}

impl Headers {
    /// The total number of headers
    ///
    /// Note that this may be _more than_ the number of keys as headers may be repeated
    pub fn len(&self) -> usize {
        self.len as _
    }

    /// Returns true if there are no headers
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Get a header by name
    pub fn get(&self, key: impl AsRef<str>) -> Option<&Header> {
        self.inner.get(key.as_ref())
    }

    /// An iterator over the headers
    pub fn iter(&self) -> Iter<'_> {
        Iter {
            inner: self.inner.values(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Iter<'a> {
    inner: hash_map::Values<'a, String, Header>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Header;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

/// The response to a `HEAD` command
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Head {
    /// The number of the article unique to a particular newsgroup
    pub number: ArticleNumber,
    /// The unique message id for the article
    pub message_id: String,
    /// The headers for the article
    pub headers: Headers,
}

impl TryFrom<&RawResponse> for Head {
    type Error = Error;

    fn try_from(resp: &RawResponse) -> Result<Self> {
        err_if_not_kind(resp, Kind::Head)?;

        let (number, message_id) = process_article_first_line(&resp)?;

        let data_blocks = resp
            .data_blocks
            .as_ref()
            .ok_or_else(Error::missing_data_blocks)?;

        let (_, headers) = take_headers(&data_blocks.payload())
            .map_err(|e| Error::invalid_data_blocks(format!("{}", e)))?;

        Ok(Self {
            number,
            message_id,
            headers,
        })
    }
}
