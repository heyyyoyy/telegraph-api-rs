# telegraph-api-rs

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
