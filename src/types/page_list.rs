use serde::Deserialize;

use super::page::Page;


#[derive(Deserialize)]
pub struct PageList {
    total_count: u32,
    pages: Vec<Page>
}
