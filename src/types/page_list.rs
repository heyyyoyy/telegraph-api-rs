use serde::Deserialize;

use super::page::Page;


#[derive(Deserialize)]
pub struct PageList {
    pub total_count: u32,
    pub pages: Vec<Page>
}
