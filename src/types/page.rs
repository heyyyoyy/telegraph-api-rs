use serde::Deserialize;

use super::node::Node;


#[derive(Deserialize)]
pub struct Page {
    path: String,
    url: String,
    title: String,
    description: String,
    author_name: Option<String>,
    author_url: Option<String>,
    image_url: Option<String>,
    content: Option<Vec<Node>>,
    views: u32,
    can_edit: Option<bool>
}
