use std::str::FromStr;

use log::*;

use crate::error::{Error, Result};
use crate::types::prelude::*;

/// Parse a generic field from the first line of an NNTP Response
///
/// 1. The provided field name will be used in the error message if parsing fails
/// 2. This will advance the provided iterator
pub(crate) fn parse_field<'a, T: FromStr>(
    iter: &mut impl Iterator<Item = &'a str>,
    name: impl AsRef<str>,
) -> Result<T> {
    let name = name.as_ref();
    iter.next()
        .ok_or_else(|| Error::missing_field(name))
        .and_then(|s| s.parse().map_err(|_| Error::parse_error(name)))
}

/// Return a deserialization error if the response does match the desired error code
pub(crate) fn err_if_not_kind(resp: &RawResponse, desired: Kind) -> Result<()> {
    if resp.code != ResponseCode::Known(desired) {
        Err(Error::Deserialization(format!(
            "Invalid response code {}",
            resp.code()
        )))
    } else {
        Ok(())
    }
}

pub(crate) fn process_article_first_line(resp: &RawResponse) -> Result<(ArticleNumber, String)> {
    let lossy = resp.first_line_to_utf8_lossy();
    let mut iter = lossy.split_whitespace();

    iter.next(); // skip response code since we already parsed it

    let number: ArticleNumber = parse_field(&mut iter, "article-number")?;
    // https://tools.ietf.org/html/rfc3977#section-9.8
    let message_id: String = parse_field(&mut iter, "message-id")?;

    trace!(
        "Parsed article-number {} and message-id {} from Article",
        number,
        message_id
    );

    Ok((number, message_id))
}
