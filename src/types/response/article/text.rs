use std::str::Utf8Error;

use crate::types::response::{BinaryArticle, Headers};

/// A text Netnews article
///
/// The ext articles must be UTF-8
///
///
#[derive(Clone, Debug)]
pub struct TextArticle {
    pub(crate) headers: Headers,
    pub(crate) body: Vec<String>,
}

impl TextArticle {
    /// The headers on the article
    pub fn headers(&self) -> &Headers {
        self.headers()
    }

    /// Create a text article from a binary one
    pub fn from_binary(b: &BinaryArticle) -> std::result::Result<Self, Utf8Error> {
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
    pub fn lines(&self) -> Lines {
        Lines(self.body.iter())
    }
}

/// Created with [`TextArticle::lines`]
pub struct Lines<'a>(std::slice::Iter<'a, String>);

impl<'a> Iterator for Lines<'a> {
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
