use serde::Deserialize;

use std::collections::HashMap;

use super::node::Node;


#[derive(Deserialize, PartialEq, Eq, Hash)]
enum NodeElementAttr {
    #[serde(alias = "href")]
    Href(String),
    #[serde(alias = "src")]
    Src(String)
}

#[derive(Deserialize)]
pub struct NodeElement {
    tag: String,
    attrs: Option<HashMap<NodeElementAttr, String>>,
    children: Option<Vec<Node>>
}
