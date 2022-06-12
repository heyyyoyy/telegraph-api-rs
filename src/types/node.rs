use serde::Deserialize;

use super::NodeElement;


#[derive(Deserialize)]
#[serde(untagged)]
enum NodeContent {
    NodeElement(NodeElement),
    String(String)
}

#[derive(Deserialize)]
pub struct Node(NodeContent);
