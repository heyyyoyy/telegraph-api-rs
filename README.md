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
telegraph-api-rs = "0.1.2"
```

## Create account
```rust
use telegraph_api_rs::{Telegraph, Request};

let telegraph = Telegraph::new();
let account = telegraph.create_account()
.short_name("Short name")
.author_name("Author name")
.send()
.unwrap();
```

## Upload media files
```rust
use telegraph_api_rs::Telegraph;

let telegraph = Telegraph::new();
let files = vec!["1.jpg", "2.png"];
let media = telegraph.upload(&files);
```
You can upload media with custom client
```rust
use telegraph_api_rs::Telegraph;
use reqwest::blocking::Client;
 
let client = Client::new();
let files = vec!["1.jpg", "2.png"];
let media = Telegraph::upload_with(&client, &files);
```
More examples in the [documentation](https://docs.rs/telegraph-api-rs)
