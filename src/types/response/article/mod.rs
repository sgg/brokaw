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
