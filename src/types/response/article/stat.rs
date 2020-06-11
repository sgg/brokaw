use std::convert::TryFrom;

use crate::error::{Error, Result};
use crate::types::prelude::*;
use crate::types::response::util::{err_if_not_kind, process_article_first_line};

/// Article metadata returned by [`STAT`](https://tools.ietf.org/html/rfc3977#section-6.2.4)
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Stat {
    /// The number of the article unique to a particular newsgroup
    pub number: ArticleNumber,
    /// The unique message id for the article
    pub message_id: String,
}

impl TryFrom<&RawResponse> for Stat {
    type Error = Error;

    fn try_from(resp: &RawResponse) -> Result<Self> {
        err_if_not_kind(resp, Kind::ArticleExists)?;

        let (number, message_id) = process_article_first_line(resp)?;

        Ok(Self { number, message_id })
    }
}
