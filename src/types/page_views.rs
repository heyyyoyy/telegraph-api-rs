use serde::Deserialize;

use super::TelegraphType;


/// This object represents the number of page views for a Telegraph article.
#[derive(Deserialize, Default, Debug)]
pub struct PageViews {
    /// Number of page views for the target page.
    pub views: u32
}


impl TelegraphType for PageViews {}


#[cfg(test)]
mod tests {
    use serde_json;
    use super::PageViews;

    #[test]
    fn page_views_deserialize() {
        let page_views_str = r#"
            {
                "views": 20
            }
        "#;
        let page_views: PageViews = serde_json::from_str(page_views_str).unwrap_or_default();
        assert_eq!(page_views.views, 20);
    }
}
