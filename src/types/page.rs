use serde::Deserialize;

use super::{node::Node, TelegraphType};


/// Object represents a page on Telegraph.
#[derive(Deserialize, Default, Debug)]
pub struct Page {
    /// Path to the page.
    pub path: String,
    /// URL of the page.
    pub url: String,
    /// Title of the page.
    pub title: String,
    /// Description of the page.
    pub description: String,
    /// Name of the author, displayed below the title.
    pub author_name: Option<String>,
    /// Profile link, opened when users click on the 
    /// author's name below the title.
    /// Can be any link, not necessarily to a Telegram profile or channel.
    pub author_url: Option<String>,
    /// Image URL of the page.
    pub image_url: Option<String>,
    /// [Content][`Node`] of the page.
    pub content: Option<Vec<Node>>,
    /// Number of page views for the page.
    pub views: u32,
    /// Only returned if access_token passed. 
    /// True, if the target Telegraph account can edit the page.
    pub can_edit: Option<bool>
}


impl TelegraphType for Page {}


#[cfg(test)]
mod tests {
    use serde_json;

    use super::Page;

    #[test]
    fn page_deserialize() {
        let page_str = r#"
        {
            "path": "path test",
            "url": "url test",
            "title": "title test",
            "description": "description test",
            "views": 10,
            "can_edit": true
        }"#;
        let page: Page = serde_json::from_str(page_str).unwrap_or_default();
        assert_eq!(page.path, "path test");
        assert_eq!(page.url, "url test");
        assert_eq!(page.title, "title test");
        assert_eq!(page.description, "description test");
        assert_eq!(page.views, 10);
        assert!(page.can_edit.unwrap_or(false));
    }
}
