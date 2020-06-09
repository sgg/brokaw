use crate::error::Result;
use crate::types::response::{BinaryArticle, Headers};

/// A text Netnews article
///
/// Unlike [`BinaryArticle`] a `TextArticle` must have a UTF-8 body.
///
/// The following methods can be used to convert from a [`BinaryArticle`]
///
/// * [`from_binary`](`Self::from_binary`) is fallible as it performs UTF-8 checks
/// * [`from_binary_lossy`](Self::from_binary_lossy) is infallible but will replace
/// non UTF-8 characters with placeholders
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextArticle {
    pub(crate) headers: Headers,
    pub(crate) body: Vec<String>,
}

impl TextArticle {
    /// The headers on the article
    pub fn headers(&self) -> &Headers {
        &self.headers
    }

    /// Create a text article from a
    pub fn from_binary(b: &BinaryArticle) -> Result<Self> {
        b.to_text()
    }

    /// Create a text article from a binary one,
    /// replacing invalid UTF-8 characters with placeholders
    pub fn from_binary_lossy(b: &BinaryArticle) -> Self {
        b.to_text_lossy()
    }

    /// An iterator over the lines in the body of the article
    ///
    /// Each line _will not_ include the CRLF terminator
    pub fn lines(&self) -> Lines<'_> {
        Lines(self.body.iter())
    }
}

/// Created with [`TextArticle::lines`]
#[derive(Clone, Debug)]
pub struct Lines<'a>(std::slice::Iter<'a, String>);

impl<'a> Iterator for Lines<'a> {
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
