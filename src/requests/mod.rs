//! Available methods
//!
//! # Result
//! Always has a Boolean field `ok`. If `ok` equals `true`, the request was successful, 
//! and the result of the query can be found in the `result` field. 
//! All queries must be made using UTF-8.
//! 
//! # Error
//! 
//! In case of an unsuccessful request, `ok` equals `false`, 
//! and the `error` is explained in the error 
//! [`TelegraphError::ApiError`][crate::error::TelegraphError] (e.g. SHORT_NAME_REQUIRED).



mod create_account;
mod edit_account_info;
mod get_account_info;
mod revoke_access_token;

mod create_page;
mod edit_page;
mod get_page;
mod get_page_list;
mod get_views;

use serde::{Serialize, Serializer};
use serde::ser;

pub use create_account::{CreateAccount, ShortName, NoShortName};
pub use edit_account_info::EditAccountInfo;
pub use get_account_info::GetAccountInfo;
pub use revoke_access_token::RevokeAccessToken;

pub use create_page::CreatePage;
pub use edit_page::EditPage;
pub use get_page::GetPage;
pub use get_page_list::GetPageList;
pub use get_views::GetViews;

use crate::types::Node;


/// Access token of the empty state type
#[derive(Default)]
pub struct NoAccessToken;

/// Access token of the filled state type
#[derive(Serialize)]
pub struct AccessToken(String);

/// Title of the empty state type 
#[derive(Default)]
pub struct NoTitle;

/// Title of the filled state type
#[derive(Serialize)]
pub struct Title(String);

/// Title of the empty state type 
#[derive(Default, Serialize)]
pub struct NoContent;

/// Title of the filled state type
#[derive(Serialize)]
pub struct Content(Vec<Node>);

/// Path of the empty state type
#[derive(Default)]
pub struct NoPath;

/// Path of the filled state type
#[derive(Serialize)]
pub struct Path(String);

/// Custom serializer for method bulders
pub struct ApiFieldSerializer;

impl ApiFieldSerializer {
    fn serialize<T: Serialize, S: Serializer>(
        value: &T,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    {
        match serde_json::to_string(value) {
            Ok(json) => serializer.serialize_str(&json),
            Err(_) => Err(ser::Error::custom("Failed to serialize value to json")),
        }
    }
}
