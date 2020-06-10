use crate::error::Result;
use crate::types::prelude::*;

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
    pub(crate) number: ArticleNumber,
    pub(crate) message_id: String,
    pub(crate) headers: Headers,
    pub(crate) body: Vec<String>,
}

impl TextArticle {
    /// The number of the article relative to the group it was retrieved from
    pub fn number(&self) -> ArticleNumber {
        self.number
    }

    /// The message id of the article
    pub fn message_id(&self) -> &str {
        &self.message_id
    }

    /// The headers on the article
    pub fn headers(&self) -> &Headers {
        &self.headers
    }

    /// Return the body of the article
    pub fn body(&self) -> &[String] {
        self.body.as_slice()
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
