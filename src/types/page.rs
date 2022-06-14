use serde::Deserialize;

use super::node::Node;


#[derive(Deserialize, Default)]
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
