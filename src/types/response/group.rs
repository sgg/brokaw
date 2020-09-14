use std::convert::TryFrom;

use crate::error::{Error, Result};
use crate::types::prelude::*;
use crate::types::response::util::{err_if_not_kind, parse_field};

/// Newsgroup metadata returned by [`GROUP`](https://tools.ietf.org/html/rfc3977#section-6.1.1)
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Group {
    /// The _estimated_ number of articles in the group
    pub number: ArticleNumber,
    /// The lowest reported article number
    pub low: ArticleNumber,
    /// The highest reported article number
    pub high: ArticleNumber,
    /// The name of the group
    pub name: String,
}

impl TryFrom<&RawResponse> for Group {
    type Error = Error;

    fn try_from(resp: &RawResponse) -> Result<Self> {
        err_if_not_kind(resp, Kind::GroupSelected)?;

        let lossy = resp.first_line_to_utf8_lossy();
        let mut iter = lossy.split_whitespace();

        // pop the response code
        iter.next()
            .ok_or_else(|| Error::missing_field("response code"))?;

        let number = parse_field(&mut iter, "number")?;
        let low = parse_field(&mut iter, "low")?;
        let high = parse_field(&mut iter, "high")?;
        let name = parse_field(&mut iter, "name")?;
        Ok(Self {
            number,
            low,
            high,
            name,
        })
    }
}
