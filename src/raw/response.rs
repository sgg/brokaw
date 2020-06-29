use std::borrow::Cow;
use std::str::{from_utf8, from_utf8_unchecked};

use crate::error::Error;

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

    /// Return the first line of the response without the response code
    pub fn first_line_without_code(&self) -> &[u8] {
        // n.b. this should be infallible barring bugs in the response parsing layer
        &self.first_line[4..]
    }

    /// Converts a response into an error if it does not match the provided status
    pub fn fail_unless(self, desired: impl Into<ResponseCode>) -> Result<RawResponse, Error> {
        if self.code() != desired.into() {
            Err(Error::failure(self))
        } else {
            Ok(self)
        }
    }

    /// Lossily convert the first line to UTF-8
    pub fn first_line_to_utf8_lossy(&self) -> Cow<'_, str> {
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

/// The [Multi-line Data Blocks](https://tools.ietf.org/html/rfc3977#section-3.1.1)
/// portion of an NNTP response
///
///
/// # Usage
///
/// [`DataBlocks::payload`](Self::payload) returns the raw bytes in the payload
/// * [`DataBlocks::lines`](Self::lines) returns an iterator over the lines within the block
/// * [`DataBlocks::unterminated`](Self::unterminated) returns an iterator over the lines with the
/// CRLF terminator and the final `.` line of the response stripped
#[derive(Clone, Debug)]
pub struct DataBlocks {
    pub(crate) payload: Vec<u8>,
    pub(crate) line_boundaries: Vec<(usize, usize)>,
}

impl DataBlocks {
    /// Return the raw contained by the payload of the Datablocks
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    /// A convenience function that simply calls [`from_utf8`]
    pub fn payload_as_utf8(&self) -> Result<&str, std::str::Utf8Error> {
        from_utf8(&self.payload)
    }

    /// An iterator over the lines within the data block
    pub fn lines(&self) -> Lines<'_> {
        Lines {
            data_blocks: self,
            inner: self.line_boundaries.iter(),
        }
    }

    /// An iterator over the unterminated data block
    ///
    /// 1. Lines yielded by this iterator WILL NOT include the CRLF terminator
    /// 2. The final line of the message containing only `.` will not be returend
    pub fn unterminated(&self) -> Unterminated<'_> {
        Unterminated {
            inner: self.lines(),
        }
    }

    /// The number of lines
    pub fn lines_len(&self) -> usize {
        self.line_boundaries.len()
    }

    /// The number of bytes in the data block
    pub fn payload_len(&self) -> usize {
        self.payload.len()
    }

    /// Returns true if there are no lines
    pub fn is_empty(&self) -> bool {
        self.line_boundaries.is_empty()
    }
}

/// An iterator over the data blocks within a response
#[derive(Clone, Debug)]
pub struct Lines<'a> {
    data_blocks: &'a DataBlocks,
    inner: std::slice::Iter<'a, (usize, usize)>,
}

impl<'a> Iterator for Lines<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((start, end)) = self.inner.next() {
            Some(&self.data_blocks.payload[*start..*end])
        } else {
            None
        }
    }
}

/// An iterator created by [`DataBlocks::unterminated`]
#[derive(Clone, Debug)]
pub struct Unterminated<'a> {
    inner: Lines<'a>,
}

impl<'a> Iterator for Unterminated<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some(line) if line == b".\r\n" => None,
            Some(line) => Some(&line[..line.len() - 2]),
            None => None,
        }
        //let foo: ()= self.data_blocks.lines().take_while(|line| line != b".\r\n");
        //unimplemented!()
    }
}
