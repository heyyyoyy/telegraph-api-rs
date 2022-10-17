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
use reqwest::{blocking::Client, Error};
use serde::{Serialize, Serializer};
use serde::ser;

pub use create_account::CreateAccount;
pub use edit_account_info::EditAccountInfo;
pub use get_account_info::GetAccountInfo;
pub use revoke_access_token::RevokeAccessToken;

pub use create_page::CreatePage;
pub use edit_page::EditPage;
pub use get_page::GetPage;
pub use get_page_list::GetPageList;
pub use get_views::GetViews;


pub trait Request {
    type MethodBuilder;
    type Response;

    fn new (client: Rc<Client>, method_name: Rc<String>) -> Self::MethodBuilder;
    fn send(&self) -> Result<Self::Response, Error>;
}


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


pub struct RequestBuilder;

impl RequestBuilder {
    pub fn build<T> (client: Rc<Client>, method_name: Rc<String>) -> T
    where T: Request + Request<MethodBuilder = T>
    {
        T::new(client, method_name)
    }
}
