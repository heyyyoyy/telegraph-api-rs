use serde::{Deserialize, Serialize};

use super::node::Node;


#[derive(Deserialize, Serialize, Debug)]
pub enum NodeElementAttr {
    #[serde(rename = "id")]
    ID(String),
    #[serde(rename = "href")]
    HREF(String),
    #[serde(rename = "src")]
    SRC(String)
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct NodeElement {
    pub tag: String,
    pub attrs: Option<NodeElementAttr>,
    pub children: Option<Vec<Node>>
}


#[cfg(test)]
mod tests {
    use serde_json;

    use super::{NodeElement, Node, NodeElementAttr};

    #[test]
    fn node_elements_deserialize() {
        let node_el_str = r#"
        {
            "tag": "p",
            "children": ["Hello world!"]
        }"#;
        let node_element: NodeElement = serde_json::from_str(node_el_str).unwrap_or_default();
        assert_eq!(node_element.tag, "p");

        let node = node_element.children.unwrap_or_default().into_iter().nth(0);
        let el = if let Some(Node::String(el)) = node {
            el
        } else {
            String::new()
        };

        assert_eq!(el, "Hello world!");    
    }

    #[test]
    fn node_elements_with_attr_deserialize() {
        let node_el_str = r#"
        {
            "tag": "p",
            "attrs": {"href": "link1"}
        }"#;
        let node_element: NodeElement = serde_json::from_str(node_el_str).unwrap_or_default();
        let node_attr_element = if let Some(NodeElementAttr::HREF(el)) = node_element.attrs {
            el
        } else {
            "".into()
        };

        assert_eq!(node_attr_element, "link1"); 
    }
}
