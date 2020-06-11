use std::collections::{hash_map, HashMap, HashSet};
use std::convert::TryFrom;
use std::fmt;

use crate::error::{Error, Result};
use crate::types::prelude::*;
use crate::types::response::util::err_if_not_kind;

/// Server capabilities
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Capabilities(HashMap<String, Capability>);

/// A capability advertised by the server
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Capability {
    pub name: String,
    pub args: Option<HashSet<String>>,
}

impl Capabilities {
    /// An iterator over the capabilities
    pub fn iter(&self) -> Iter<'_> {
        Iter {
            inner: self.0.values(),
        }
    }

    /// Retrieve a capability if it exists
    pub fn get(&self, key: impl AsRef<str>) -> Option<&Capability> {
        self.0.get(key.as_ref())
    }
}

impl fmt::Display for Capability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)?;
        if let Some(args) = self.args.as_ref() {
            args.iter()
                .map(|arg| {
                    write!(f, " {}", arg)?;
                    Ok::<_, fmt::Error>(())
                })
                .for_each(drop);
        }

        Ok(())
    }
}

/// Created by [`Capabilities::iter`]
#[derive(Clone, Debug)]
pub struct Iter<'a> {
    inner: hash_map::Values<'a, String, Capability>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Capability;

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
            .map(DataBlocks::unterminated)?;

        let capabilities: HashMap<String, Capability> = db_iter
            .map(String::from_utf8_lossy)
            .map(|entry| {
                let mut entry_iter = entry.split_whitespace().peekable();
                let label = entry_iter
                    .next()
                    .map(ToString::to_string)
                    .ok_or_else(|| Error::de("Entry does not have a label"))?;

                let args = if entry_iter.peek().is_some() {
                    Some(entry_iter.map(ToString::to_string).collect::<HashSet<_>>())
                } else {
                    None
                };

                let cap = Capability {
                    name: label.clone(),
                    args,
                };

                Ok((label, cap))
            })
            .collect::<Result<_>>()?;

        Ok(Self(capabilities))
    }
}
