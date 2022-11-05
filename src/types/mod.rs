//! Available types
//! 
//! All types used in the Telegraph API responses are 
//! represented as Struct implemented [`TelegraphType`]. 
//! Optional fields may be not returned when irrelevant.

mod account;
mod page;
mod node;
mod node_element;
mod page_list;
mod page_views;
mod media;


use crate::TelegraphError;

pub use self::account::{Account, AccountField};
pub use self::page::Page;
pub use self::node::Node;
pub use self::node_element::{NodeElement, NodeTag, NodeElementAttr};
pub use self::page_list::PageList;
pub use self::page_views::PageViews;
#[cfg(feature = "upload")]
pub use self::media::{UploadResult, Media};


use serde::Deserialize;


/// Trait for Telegraph API [types][crate::types]
pub trait TelegraphType {}


/// Telegraph API response 
#[derive(Deserialize, Debug)]
pub struct TelegraphResult<T> 
where T: TelegraphType
{
    /// If `ok` equals true, the request was successful, 
    /// and the result of the query can be found in the `result`
    pub ok: bool,
    /// API result 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<T>,
    /// If `ok` equals false, and the error is explained 
    /// in the error field (e.g. SHORT_NAME_REQUIRED)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<TelegraphError>
}
