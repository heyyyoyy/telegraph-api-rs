use serde::Deserialize;

use super::node::Node;


#[derive(Deserialize)]
pub struct Page {
    pub path: String,
    pub url: String,
    pub title: String,
    pub description: String,
    pub author_name: Option<String>,
    pub author_url: Option<String>,
    pub image_url: Option<String>,
    pub content: Option<Vec<Node>>,
    pub views: u32,
    pub can_edit: Option<bool>
}
