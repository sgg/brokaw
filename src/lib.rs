#![deny(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unconditional_recursion
)]

//! TODO(docs) Brokaw

#[cfg(doctest)]
doc_comment::doctest!("../README.md");

/// The high-level client and business logic
///
/// TODO(details w/ examples)
pub mod client;

/// Error and Result types returned by the Brokaw client API
pub mod error;

/// Low level connection/stream APIs
///
/// These deal with raw NNTP connections and byte responses.
/// Consider using the higher level [`client`] APIs unless you have special requirements
pub mod raw;

/// Types for sending commands, and reading responses
pub mod types;
