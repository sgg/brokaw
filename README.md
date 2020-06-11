<h1 align="center">Brokaw</h1>
<div align="center">
  <strong>📰 An NNTP client liberary. More at 11! 📰</strong>
</div>

<br />

<div align="center">
  <a href="https://github.com/sgg/brokaw/actions">
  <!-- Actions Status -->
    <img src="https://github.com/sgg/brokaw/workflows/Rust/badge.svg"
      alt="GitHub Actions" />
  </a>
  <!-- Crates version -->
  <a href="https://crates.io/crates/brokaw">
    <img src="https://img.shields.io/crates/v/brokaw"
    alt="Crates.io version" />
  </a>
  <!-- docs.rs docs -->
  <a href="https://docs.rs/brokaw">
    <img src="https://img.shields.io/badge/docs.rs-latest-blue.svg?"
      alt="docs.rs docs" />
  </a>
</div>


Brokaw is a a typed Usenet library for the dozens of people still reading Netnews. It is very much in development and provides **no guarantees about stability**.

Brokaw (mostly) implements [RFC 3977](https://tools.ietf.org/html/rfc3977) and several popular extensions.

## Getting Started

```toml
[dependencies]
brokaw = "0.0.1"
```

```rust
use brokaw::client::ClientConfig;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ClientConfig::default().connect(("news.mozilla.org", 119))?;

    client.capabilities().iter()
        .for_each(|c| println!("{}", c));

    Ok(())
}
```

Check out in the repo [the examples](./examples) as well!

## Features

* TLS (aka `NNTPS`) courtesy of [`native-tls`](https://crates.io/crates/native-tls)
* A high-level client API (`NntpClient`) for simple interactions with news servers
* A low-level connection API (`NntpConnection`) for more specialized use cases
* `AUTHINFO USER/PASS` Authentication ([RFC 4643]
* Typed commands and responses 
* ~All~ Most commands in [RFC 3977] (`POST`, `NEWGROUP`, `NEWNEWS`, and `LISTGROUP` have yet to be implemented)

## Missing Features

* Compression (RFC 8054, Astraweb, Giganews, etc)
* STARTTLS ([RFC 4642](https://tools.ietf.org/html/rfc4642))
* SASL Authentication ([RFC 4643])
* Most of [RFC 2980]. `XHDR` and `XOVER` are supported
* Connection pools, fine grained connection tuning
* Async connection/client
* Article posting

[RFC 2980]: (https://tools.ietf.org/html/rfc4643)
[RFC 3977]: https://tools.ietf.org/html/rfc3977
[RFC 4642]: https://tools.ietf.org/html/rfc4642
[RFC 4643]: (https://tools.ietf.org/html/rfc4643)
