use serde::Deserialize;

use super::{page::Page, TelegraphType};


#[derive(Deserialize, Default, Debug)]
pub struct PageList {
    pub total_count: u32,
    pub pages: Vec<Page>
}


impl TelegraphType for PageList {}


#[cfg(test)]
mod tests {
    use serde_json;

    use super::PageList;

    #[test]
    fn page_list_deserialize() {
        let page_list_str = r#"
        {
            "total_count": 1,
            "pages": [{
                "path": "path test",
                "url": "url test",
                "title": "title test",
                "description": "description test",
                "views": 10,
                "can_edit": true
            }]
        }"#;
        let page: PageList = serde_json::from_str(page_list_str).unwrap_or_default();
        assert_eq!(page.total_count, 1);
        assert_eq!(page.pages.len(), 1);
        assert_eq!(page.pages[0].path, "path test");
        assert_eq!(page.pages[0].url, "url test");
    }
}