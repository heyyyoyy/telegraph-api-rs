mod account;
mod page;
mod node;
mod node_element;
mod page_list;
mod page_views;


use crate::TelegraphError;

pub use self::account::{Account, AccountField};
pub use self::page::Page;
pub use self::node::Node;
pub use self::node_element::NodeElement;
pub use self::page_list::PageList;
pub use self::page_views::PageViews;


use serde::Deserialize;


/// Trait for Telegraph Api (types)[crate::types]
pub trait TelegraphType {}


/// Telegraph Api response 
#[derive(Deserialize, Debug)]
pub struct TelegraphResult<T> 
where T: TelegraphType
{
    /// If `ok` equals true, the request was successful, 
    /// and the result of the query can be found in the `result`
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Api result 
    pub result: Option<T>,
    /// If `ok` equals false, and the error is explained 
    /// in the error field (e.g. SHORT_NAME_REQUIRED)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<TelegraphError>
}
