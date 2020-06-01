use std::borrow::Cow;
use std::str::{from_utf8, from_utf8_unchecked};

use crate::types::response_code::ResponseCode;

/// A response returned by the low-level [`NntpConnection`](super::connection::NntpConnection)
///
/// 1. The contents are guaranteed to be represent a syntactically valid NNTP response
/// 2. The contents ARE NOT guaranteed to be UTF-8 as the NNTP does not require contents be UTF-8.
#[derive(Clone, Debug)]
pub struct RawResponse {
    pub(crate) code: ResponseCode,
    pub(crate) first_line: Vec<u8>,
    pub(crate) data_blocks: Option<DataBlocks>,
}

impl RawResponse {
    /// The response code
    pub fn code(&self) -> ResponseCode {
        self.code
    }

    /// Return true if this response is a multi-line response and contains a data block section
    pub fn has_data_blocks(&self) -> bool {
        match self.data_blocks {
            Some(_) => true,
            _ => false,
        }
    }

    /// Return multi-line data blocks
    pub fn data_blocks(&self) -> Option<&DataBlocks> {
        self.data_blocks.as_ref()
    }

    /// Return the first line of the response
    pub fn first_line(&self) -> &[u8] {
        &self.first_line
    }

    // Return the first line of the response without the response code
    pub fn first_line_without_code(&self) -> &[u8] {
        // n.b. this should be infallible barring bugs in the response parsing layer
        &self.first_line[4..]
    }

    /// Return the initial response payload as a UTF-8 str
    pub fn first_line_as_utf8(&self) -> Result<&str, std::str::Utf8Error> {
        from_utf8(&self.first_line).map(|s| s.trim())
    }

    pub fn first_line_to_utf8_lossy(&self) -> Cow<str> {
        String::from_utf8_lossy(&self.first_line)
    }

    /// Convert the initial response payload into UTF-8 without checking
    ///
    /// # Safety
    ///
    /// This function is unsafe because NNTP responses are NOT required to be UTF-8.
    /// This call simply calls [`from_utf8_unchecked`] under the hood.
    ///
    pub unsafe fn first_line_as_utf8_unchecked(&self) -> &str {
        from_utf8_unchecked(&self.first_line)
    }
}

/// The multi-line data block portion of an NNTP response
///
/// [`DataBlocks::iter`] can be used to create iterators over the blocks
///
/// https://tools.ietf.org/html/rfc3977#section-3.1.1
#[derive(Clone, Debug)]
pub struct DataBlocks {
    pub(crate) payload: Vec<u8>,
    pub(crate) line_boundaries: Vec<(usize, usize)>,
}

impl DataBlocks {
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    pub fn payload_as_utf8(&self) -> Result<&str, std::str::Utf8Error> {
        from_utf8(&self.payload)
    }

    pub fn iter(&self) -> Iter {
        Iter {
            datablocks: self,
            inner: self.line_boundaries.iter(),
        }
    }

    pub fn len(&self) -> usize {
        self.line_boundaries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.line_boundaries.is_empty()
    }
}

pub struct Iter<'a> {
    datablocks: &'a DataBlocks,
    inner: std::slice::Iter<'a, (usize, usize)>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((low, high)) = self.inner.next() {
            Some(&self.datablocks.payload[*low..*high])
        } else {
            None
        }
    }
}
