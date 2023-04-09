# telegraph-api-rs

![Crates.io](https://img.shields.io/crates/v/telegraph-api-rs?style=plastic)
![Crates.io](https://img.shields.io/crates/l/telegraph-api-rs?style=plastic)
[![CI](https://github.com/heyyyoyy/telegraph-api-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/heyyyoyy/telegraph-api-rs/actions/workflows/rust.yml)
[![docs.rs](https://img.shields.io/docsrs/telegraph-api-rs?style=plastic)](https://docs.rs/telegraph-api-rs/0.2.0/telegraph_api_rs/)

Rust implementation of [Telegraph API](https://telegra.ph/api)

# Quick start

Add dependency to the `Cargo.toml`
```toml
[dependencies]
telegraph-api-rs = "0.2.0"
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

## Edit account
```rust
let edited_account = telegraph.edit_account_info()
    .access_token(token)
    .author_name("Author name 2")
    .send()
    .unwrap();
```

## Get account info
```rust
let account_info = telegraph.get_account_info()
    .access_token(token)
    .fields(vec![AccountField::ShortName, AccountField::AuthorUrl])
    .send()
    .unwrap();
```

## Revoke access token
```rust
let account = telegraph.revoke_access_token()
    .access_token(token)
    .send()
    .unwrap();
```

## Create page
```rust
let content = r#"
[
    {
        "tag": "h3",
        "children": ["Hello world"]
    },
    {
        "tag": "h4",
        "children": ["Title"]
    },
    {
        "tag": "p",
        "children": [
            {
                "tag": "ul",
                "children": ["Some text"]
            }
        ]
    }
]
"#;

let cont = build_content(content).unwrap();

let page = telegraph.create_page()
    .access_token(token)
    .title("Hello world")
    .content(cont)
    .return_content(true)
    .send()
    .unwrap();
```

## Edit page
```rust
let new_content = r#"
[
    {
    "tag": "h3",
        "children": ["Hello world"]
    },
    {
        "tag": "h4",
        "children": ["Title"]
    },
    {
        "tag": "p",
        "children": [
            {
                "tag": "ul",
                "children": ["Some text"]
            },
            {
                "tag": "ul",
                "children": ["Some text 2"]
            }
        ]
    }
]
"#;

let new_cont = build_content(new_content).unwrap();

let edited_page = telegraph.edit_page()
    .access_token(token)
    .title(&page.title)
    .path(&page.path)
    .content(new_cont)
    .return_content(true)
    .send()
    .unwrap();
```

## Get page
```rust
let get_page = telegraph.get_page()
    .path(&page.path)
    .send()
    .unwrap();
```

## Get page list
```rust
let page_list = telegraph.get_page_list()
    .access_token(token)
    .limit(2)
    .send()
    .unwrap();
```

## Get views
```rust
let count = telegraph.get_views()
    .path(&page_list.pages[0].path)
    .year(2023)
    .month(10)
    .day(15)
    .hour(24)
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
