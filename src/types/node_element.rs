use serde::{Deserialize, Serialize};

use super::node::Node;


#[allow(missing_docs)]
/// Available tags.
#[derive(Deserialize, Serialize, Debug)]
pub enum NodeTag {
    #[serde(rename = "a")]
    A(String),
    #[serde(rename = "aside")]
    Aside(String),
    #[serde(rename = "b")]
    B(String),
    #[serde(rename = "blockquote")]
    Blockquote(String),
    #[serde(rename = "br")]
    Br(String),
    #[serde(rename = "code")]
    Code(String),
    #[serde(rename = "em")]
    Em(String),
    #[serde(rename = "figcaption")]
    Figcaption(String),
    #[serde(rename = "figure")]
    Figure(String),
    #[serde(rename = "h3")]
    H3(String),
    #[serde(rename = "h4")]
    H4(String),
    #[serde(rename = "hr")]
    Hr(String),
    #[serde(rename = "i")]
    I(String),
    #[serde(rename = "iframe")]
    Iframe(String),
    #[serde(rename = "img")]
    Img(String),
    #[serde(rename = "li")]
    Li(String),
    #[serde(rename = "ol")]
    Ol(String),
    #[serde(rename = "p")]
    P(String),
    #[serde(rename = "pre")]
    Pre(String),
    #[serde(rename = "s")]
    S(String),
    #[serde(rename = "strong")]
    Strong(String),
    #[serde(rename = "u")]
    U(String),
    #[serde(rename = "ul")]
    Ul(String),
    #[serde(rename = "video")]
    Video(String)
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
        let tag = if let Some(NodeTag::P(tag)) = node_element.tag{
            tag
        } else {
            String::new()
        };
        assert_eq!(tag, "p");

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
}
