//! Available methods
//! 
//! The response contains a [`Result`] with struct implemented [`Request`] trait.
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
//! [field][TelegraphError::ApiError] (e.g. SHORT_NAME_REQUIRED).



mod create_account;
mod edit_account_info;
mod get_account_info;
mod revoke_access_token;

mod create_page;
mod edit_page;
mod get_page;
mod get_page_list;
mod get_views;

use std::rc::Rc;
use reqwest::blocking::Client;
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

use crate::error::TelegraphError;
use crate::types::{TelegraphResult, TelegraphType, Node};


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

/// Trait for API methods. 
pub trait Request 
where <Self as Request>::Response: TelegraphType
{
    /// Represents a API methods struct
    type MethodBuilder;
    /// Type for [`TelegraphType`]
    type Response;

    /// Struct onstructor
    fn new (client: Rc<Client>, method_name: Rc<String>) -> Self::MethodBuilder;
    /// Sending request to API
    fn send(&self) -> Result<Self::Response, TelegraphError>;
    /// Error handling
    fn catch_api_error(resp: TelegraphResult<Self::Response>) -> Result<Self::Response, TelegraphError> {
        if !resp.ok {
            Err(resp.error.unwrap())
        } else {
            Ok(resp.result.unwrap())
        }
    }
}


/// Custom serializer for [method bulders][Request::MethodBuilder]
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


/// Request builder for available methods
pub struct RequestBuilder;

impl RequestBuilder {
    /// Call constructor of `T` struct
    pub fn build<T> (client: Rc<Client>, method_name: Rc<String>) -> T
    where T: Request + Request<MethodBuilder = T>
    {
        T::new(client, method_name)
    }
}
