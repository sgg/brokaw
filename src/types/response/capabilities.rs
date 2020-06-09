use std::collections::{HashMap, hash_map};
use std::convert::TryFrom;

use crate::error::{Error, Result};
use crate::types::prelude::*;
use crate::types::response::util::err_if_not_kind;

/// Server capabilities
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Capabilities(HashMap<String, Option<Vec<String>>>);

// FIXME(craft): newtype (or deprecate) Capabilities iterator
impl Capabilities {

    /// An iterator over the capabilities
    pub fn iter(&self) -> Iter<'_>{
        Iter {
            inner: self.0.iter()
        }
    }

    /// Retrieve a capability if it exists
    pub fn get(&self, key: impl AsRef<str>) -> Option<&Option<Vec<String>>> {
        self.0.get(key.as_ref())
    }
}

/// Created by [`Capabilities::iter`]
#[derive(Clone, Debug)]
pub struct Iter<'a> {
    inner: hash_map::Iter<'a, String, Option<Vec<String>>>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = (&'a String, &'a Option<Vec<String>>);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}


impl TryFrom<&RawResponse> for Capabilities {
    type Error = Error;

    /// Parse capabilities from a response
    ///
    /// The specific format is taken from [RFC 3977](https://tools.ietf.org/html/rfc3977#section-9.5)
    fn try_from(resp: &RawResponse) -> Result<Self> {
        err_if_not_kind(resp, Kind::Capabilities)?;

        let db_iter = resp
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
