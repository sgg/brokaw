/// Raw connection implementation
pub mod connection;

/// Low level API errors
pub mod error;

/// Response parsing logic
///
/// * The parsing is line based
/// * Naming conventions follow those in [`nom`].
/// * Any function that begins with `parse_` will fail if the provided buffer is not consumed.
pub(crate) mod parse;

/// Raw NNTP response types
pub mod response;

/// Raw TCP stream implementation
pub(crate) mod stream;
