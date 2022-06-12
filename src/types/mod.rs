mod account;
mod page;
mod node;
mod node_element;
mod page_list;
mod page_views;


pub use self::account::Account;
pub use self::page::Page;
pub use self::node::Node;
pub use self::node_element::NodeElement;
pub use self::page_list::PageList;
pub use self::page_views::PageViews;


use serde::Deserialize;


#[derive(Deserialize)]
struct TelegraphResult<T> {
    ok: bool,
    result: Option<T>,
}
