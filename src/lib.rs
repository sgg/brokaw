/// TODO(docs)
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
