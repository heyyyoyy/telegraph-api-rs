use serde::Deserialize;

#[derive(Deserialize)]
pub struct Page<T> {
    path: String,
    url: String,
    title: String,
    description: String,
    author_name: Option<String>,
    author_url: Option<String>,
    image_url: Option<String>,
    content: Vec<T>,
    views: u32,
    can_edit: Option<bool>
}