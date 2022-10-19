use serde::{Deserialize, Serialize};

use super::node::Node;


#[allow(missing_docs)]
/// Available tags.
#[derive(Deserialize, Serialize, Debug)]
pub enum NodeTag {
    #[serde(rename = "a")]
    A,
    #[serde(rename = "aside")]
    Aside,
    #[serde(rename = "b")]
    B,
    #[serde(rename = "blockquote")]
    Blockquote,
    #[serde(rename = "br")]
    Br,
    #[serde(rename = "code")]
    Code,
    #[serde(rename = "em")]
    Em,
    #[serde(rename = "figcaption")]
    Figcaption,
    #[serde(rename = "figure")]
    Figure,
    #[serde(rename = "h3")]
    H3,
    #[serde(rename = "h4")]
    H4,
    #[serde(rename = "hr")]
    Hr,
    #[serde(rename = "i")]
    I,
    #[serde(rename = "iframe")]
    Iframe,
    #[serde(rename = "img")]
    Img,
    #[serde(rename = "li")]
    Li,
    #[serde(rename = "ol")]
    Ol,
    #[serde(rename = "p")]
    P,
    #[serde(rename = "pre")]
    Pre,
    #[serde(rename = "s")]
    S,
    #[serde(rename = "strong")]
    Strong,
    #[serde(rename = "u")]
    U,
    #[serde(rename = "ul")]
    Ul,
    #[serde(rename = "video")]
    Video
}


#[allow(missing_docs)]
/// Available attrs.
#[derive(Deserialize, Serialize, Debug)]
pub enum NodeElementAttr {
    #[serde(rename = "id")]
    ID(String),
    #[serde(rename = "href")]
    Href(String),
    #[serde(rename = "src")]
    Src(String)
}


/// Object represents a DOM element node.
#[derive(Deserialize, Serialize, Default, Debug)]
pub struct NodeElement {
    /// Name of the DOM element. 
    /// Available tags [`NodeTag`]
    pub tag: Option<NodeTag>,
    /// Attributes of the DOM element. Key of object represents name of attribute, 
    /// value represents value of attribute. 
    /// Available attributes [`NodeElementAttr`]
    pub attrs: Option<NodeElementAttr>,
    /// List of child nodes for the DOM element.
    pub children: Option<Vec<Node>>
}


#[cfg(test)]
mod tests {
    use serde_json;

    use crate::types::NodeTag;

    use super::{NodeElement, Node, NodeElementAttr};

    #[test]
    fn node_elements_deserialize() {
        let node_el_str = r#"
        {
            "tag": "p",
            "children": ["Hello world!"]
        }"#;
        let node_element: NodeElement = serde_json::from_str(node_el_str).unwrap_or_default();
        let tag = node_element.tag.map(
            |tag| match tag {
                NodeTag::P => "p".to_string(),
                _ => "".to_string()
            }
        );

        assert_eq!(tag.unwrap(), "p");

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
        let node_attr_element = if let Some(NodeElementAttr::Href(el)) = node_element.attrs {
            el
        } else {
            "".into()
        };

        assert_eq!(node_attr_element, "link1"); 
    }

    #[test]
    fn node_element_serialize() {
        let node_element = vec![NodeElement {
            tag: Some(NodeTag::P), children: Some(vec![Node::String("test".to_string())]), ..NodeElement::default()
        }];
        let res = serde_json::to_string(&node_element);
        assert_eq!(res.unwrap_or("".to_string()), "[{\"tag\":\"p\",\"attrs\":null,\"children\":[\"test\"]}]".to_string()); 
    }
}
