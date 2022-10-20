# telegraph-api-rs

![Crates.io](https://img.shields.io/crates/v/telegraph-api-rs?style=plastic)
![Crates.io](https://img.shields.io/crates/l/telegraph-api-rs?style=plastic)
[![CI](https://github.com/heyyyoyy/telegraph-api-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/heyyyoyy/telegraph-api-rs/actions/workflows/rust.yml)
![docs.rs](https://img.shields.io/docsrs/telegraph-api-rs?style=plastic)

Rust implementation of [Telegraph API](https://telegra.ph/api)

# Quick start

Add dependency to the `Cargo.toml`
```toml
[dependencies]
telegraph-api-rs = "0.1.0"
```

And create account

```rust
use telegraph_api_rs::{Telegraph, Request};

let telegraph = Telegraph::new();
let account = telegraph.create_account()
.short_name("Short name")
.author_name("Author name")
.send()
.unwrap();
```

More examples in the [documentation](https://docs.rs/telegraph-api-rs)
