use serde::Deserialize;

use std::collections::HashMap;

use super::node::Node;


#[derive(Deserialize, PartialEq, Eq, Hash)]
pub enum NodeElementAttr {
    #[serde(alias = "href")]
    Href(String),
    #[serde(alias = "src")]
    Src(String)
}

#[derive(Deserialize, Default)]
pub struct NodeElement {
    pub tag: String,
    pub attrs: Option<HashMap<NodeElementAttr, String>>,
    pub children: Option<Vec<Node>>
}


#[cfg(test)]
mod tests {
    use serde_json;

    use super::{NodeElement, Node};

    #[test]
    fn node_elements_deserialize() {
        let page_views_str = r#"
        {
            "tag": "p",
            "children": ["Hello world!"]
        }"#;
        let node_element: NodeElement = serde_json::from_str(page_views_str).unwrap_or_default();
        assert_eq!(node_element.tag, "p");

        let node = node_element.children.unwrap_or_default().into_iter().nth(0);
        let el = if let Some(Node::String(el)) = node {
            el
        } else {
            String::new()
        };

        assert_eq!(el, "Hello world!");    
    }
}
