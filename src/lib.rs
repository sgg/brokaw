#![deny(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unconditional_recursion
)]

//! 🗞 Brokaw is a NNTP (Usenet) library
//!
//! # APIs
//!
//! Brokaw provides two primary APIs for interacting with NNTP servers:
//!
//! 1. The [`NntpClient`] provides a higher-level that provides a a config
//! based builder and automatic deserialization of responses into different types.
//! 2. The [`NntpConnection`] provides a lower-level abstraction that
//! only provides validation that messages adhere to NNTP's wire format.
//!
//! Brokaw additionally provides strongly typed [commands](types::command),
//! [responses](types::response), and the [`NntpCommand`](types::NntpCommand)
//! trait for implementing your own strongly typed commands.
//!
//! ---
//!
//! Please check out the [git repository](https://github.com/sgg/brokaw) examples.

#[cfg(doctest)]
doc_comment::doctest!("../README.md");

/// The high-level client and configuration API
pub mod client;

/// Error and Result types returned by the Brokaw
pub mod error;

/// Low level connection/stream APIs
///
/// These deal with raw NNTP connections and byte responses.
/// Consider using the higher level [`client`] APIs unless you have special requirements
pub mod raw;

/// Typed commands, responses, and response codes
pub mod types;

#[doc(inline)]
pub use client::{ClientConfig, NntpClient};
#[doc(inline)]
pub use raw::connection::{ConnectionConfig, NntpConnection};
#[doc(inline)]
pub use raw::Compression;
