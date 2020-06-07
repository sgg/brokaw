# Brokaw 📰 

Brokaw is a "modern" Usenet library for the modern Usenet user (there are dozens of us! Dozens!).

This project is still in early development and provides **no guarantees about stability**.

Brokaw aims to mostly implement [RFC 3977](https://tools.ietf.org/html/rfc3977) and several popular extensions.

## Getting Started (TODO)

## Examples (TODO)

## Features

* TLS (aka `NNTPS`) courtesy of [`native-tls`](https://crates.io/crates/native-tls)
* A high-level client API (`NntpClient`) for simple interactions with news servers
* A low-level connection API (`NntpConnection`) for more specialized use cases
* `AUTHINFO USER/PASS` Authentication ([RFC 4643]
* ~All~ Most commands in [RFC 3977] (`POST`, `NEWGROUP`, `NEWNEWS`, and `LISTGROUP` have yet to be implemented)

## Development Progress

### Unfinished

* Documentation and Examples
* NntpClient
    * Header/article retrieval
    * Typed capabilities support
* Craftsmanship
    * Tests and testing infra
    * Much of the `commands` module is boilerplate could be replaced w/ some proc macros
* Some kinks in the parsing logic w/ line terminators
* Iterators
    * Most of the iterators could implement ExactSizeIterator since they are not infinite

## Missing Features

* Compression (RFC 8054, Astraweb, Giganews, etc)
* STARTTLS ([RFC 4642](https://tools.ietf.org/html/rfc4642))
* SASL Authentication ([RFC 4643](https://tools.ietf.org/html/rfc4643))
* Most of [RFC 2980]. `XHDR` and `XOVER` are supported
* Connection pools, connection tuning
* Async client connections/client
* Write support (e.g. article creation, posting)

## Sharp Edges

* The NntpClient does not gracefully handle timeouts
* `LISTGROUP` is itself a sharp edge in the NNTP standard...
* Binary articles represent their data 

[RFC 2980]: (https://tools.ietf.org/html/rfc4643)
[RFC 3977]: https://tools.ietf.org/html/rfc3977
[RFC 4642]: https://tools.ietf.org/html/rfc4642
[RFC 4643]: (https://tools.ietf.org/html/rfc4643)
