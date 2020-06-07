use std::collections::{hash_map, HashMap};
use std::convert::TryFrom;
use std::str::FromStr;

use crate::error::{Error, Result};
use crate::raw::response::{DataBlocks, RawResponse};

use crate::types::response_code::ResponseCode;

mod article;

pub use article::*;

/// A response from an NNTP server
#[derive(Clone)]
pub struct Response<B>
where
    B: Clone + NntpResponseBody,
{
    code: ResponseCode,
    body: B,
}

impl<B: Clone + NntpResponseBody> Response<B> {
    /// The body of the response
    pub fn body(&self) -> &B {
        &self.body
    }
}

/// A type that may be deserialized from a [`RawResponse`]
pub trait NntpResponseBody: for<'a> TryFrom<&'a RawResponse, Error = Error> {}

/// Response to the [`GROUP`](https://tools.ietf.org/html/rfc3977#section-6.1.1) command
#[derive(Clone, Debug)]
pub struct Group {
    /// The _estimated_ number of articles in the group
    pub number: u32,
    /// The lowest reported article number
    pub low: u32,
    /// The highest reported article number
    pub high: u32,
    /// The name of the group
    pub name: String,
}

impl<B> TryFrom<&RawResponse> for Response<B>
where
    B: NntpResponseBody + Clone,
{
    type Error = Error;

    fn try_from(value: &RawResponse) -> std::result::Result<Self, Self::Error> {
        let body = B::try_from(value)?;

        Ok(Response {
            code: value.code(),
            body,
        })
    }
}

impl NntpResponseBody for Group {}

impl TryFrom<&RawResponse> for Group {
    type Error = Error;

    fn try_from(value: &RawResponse) -> Result<Self> {
        let lossy = value.first_line_to_utf8_lossy();
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

/// Server capabilities
#[derive(Clone, Debug)]
pub struct Capabilities(pub HashMap<String, Option<Vec<String>>>);

// FIXME(craft): newtype (or deprecate) Capabilities iterator
impl Capabilities {
    /// An iterator over the capabilities
    pub fn iter(&self) -> hash_map::Iter<'_, String, Option<Vec<String>>> {
        self.0.iter()
    }
}

impl NntpResponseBody for Capabilities {}

impl TryFrom<&RawResponse> for Capabilities {
    type Error = Error;

    /// Parse capabilities from a response
    ///
    /// The specific format is taken from [RFC 3977](https://tools.ietf.org/html/rfc3977#section-9.5)
    fn try_from(value: &RawResponse) -> Result<Self> {
        let db_iter = value
            .data_blocks
            .as_ref()
            .ok_or_else(|| Error::de("Missing data blocks."))
            .map(DataBlocks::lines)?;

        let capabilities: HashMap<String, Option<Vec<String>>> = db_iter
            .map(String::from_utf8_lossy)
            .map(|entry| {
                let mut entry_iter = entry.split_whitespace().peekable();
                let label = entry_iter
                    .next()
                    .map(ToString::to_string)
                    .ok_or_else(|| Error::de("Entry does not have a label"))?;

                let args = if entry_iter.peek().is_some() {
                    Some(entry_iter.map(ToString::to_string).collect::<Vec<_>>())
                } else {
                    None
                };
                Ok((label, args))
            })
            .collect::<Result<_>>()?;

        Ok(Self(capabilities))
    }
}

/// TODO: Docstring
/// Parse a generic field from the first line
///
/// The field name will be provided in the error raised if the field cannot be parsed.
///
/// The iterator provided will be advanced
pub(crate) fn parse_field<'a, T: FromStr>(
    iter: &mut impl Iterator<Item = &'a str>,
    name: impl AsRef<str>,
) -> Result<T> {
    let name = name.as_ref();
    iter.next()
        .ok_or_else(|| Error::missing_field(name))
        .and_then(|s| s.parse().map_err(|_| Error::parse_error(name)))
}
