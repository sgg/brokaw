use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;
use std::result::Result as StdResult;
use std::str::from_utf8;

use log::*;

use crate::error::{Error, Result};
use crate::raw::response::RawResponse;
use crate::types::prelude::*;
use crate::types::response::article::parse::take_headers;
use crate::types::response::parse_field;

/// Parsing logic for for article headers
mod parse;

/// Article headers
mod headers;

/// Binary articles
mod binary;

/// Text articles
mod text;

pub use binary::BinaryArticle;
pub use headers::Headers;
pub use text::TextArticle;
