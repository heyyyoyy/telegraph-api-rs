use serde::Deserialize;

#[derive(Deserialize)]
pub struct Account {
    short_name: String,
    author_name: String,
    author_url: String,
    access_token: Option<String>,
    auth_url: Option<String>,
    page_count: Option<i32>
}
