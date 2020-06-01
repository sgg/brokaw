# Brokaw 📰 

Brokaw is a modern Usenet library for the modern Usenet user (There are dozens of us! Dozens!).

This project is still in early development and provides are **no guarantees about stability**.

## Getting Started (TODO)

## Examples (TODO)

## Features

* TLS courtesy of [`native-tls`](https://crates.io/crates/native-tls)
* A high-level client API (`NntpClient`) for simple interactions with news servers
* A low-level connection API (`NntpConnection`) for more specialized use cases

## Development Progress

### Unfinished
* Header/article retrieval within the `NntpClient`
* Documentation and Examples
* Tests and testing infra
* Some kinks in the parsing logic w/ line terminators

## Missing Features

* Compression (RFC 8054, Astraweb, Giganews, etc)
* STARTTLS ([RFC 4642](https://tools.ietf.org/html/rfc4642))
* SASL Authentication ([RFC 4643](https://tools.ietf.org/html/rfc4643))
* Connection pools
* Connection tuning
* Async client

## Sharp Edges

* The NntpClient does not gracefully handle timeouts
* `LISTGROUP` is itself a sharp edge in the NNTP standard, it is unimplemented
