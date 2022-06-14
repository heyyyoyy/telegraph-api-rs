use serde::Deserialize;

use super::NodeElement;


#[derive(Deserialize)]
#[serde(untagged)]
pub enum Node {
    String(String),
    NodeElement(NodeElement)
}


#[cfg(test)]
mod tests {
    use serde_json;

    use crate::types::NodeElement;

    use super::Node;

    #[test]
    fn node_text_deserialize() {
        let json = "[\"It's text node\"]";
        let nodes: Vec<Node> = serde_json::from_str(json).unwrap_or_default();
        let el = if let Some(Node::String(el)) = nodes.into_iter().nth(0) {
            el
        } else {
            String::new()
        };
        assert_eq!(el, "It's text node")
    }

    #[test]
    fn node_element_deserialize() {
        let json = r#"
        {
            "tag": "p"
        }"#;
        let node: Node = serde_json::from_str(json).unwrap();
        let el = if let Node::NodeElement(el) = node {
            el
        } else {
            NodeElement::default()
        };
        assert_eq!(el.tag, "p");
    }
}
