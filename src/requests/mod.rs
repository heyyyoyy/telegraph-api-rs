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

pub use create_account::CreateAccount;
pub use edit_account_info::EditAccountInfo;
pub use get_account_info::GetAccountInfo;
use reqwest::{blocking::Client, Error};
pub use revoke_access_token::RevokeAccessToken;

pub use create_page::CreatePage;
pub use edit_page::EditPage;
pub use get_page::GetPage;
pub use get_page_list::GetPageList;
pub use get_views::GetViews;


pub trait ApiMethod {
    type FunctionBulder;
    type Response;

    fn new (client: Rc<Client>, method_name: Rc<String>) -> Self::FunctionBulder;
    fn send(&self) -> Result<Self::Response, Error>;
}


pub struct RequestBuilder;

impl RequestBuilder {
    pub fn build<T> (client: Rc<Client>, method_name: Rc<String>) -> T
    where T: ApiMethod + ApiMethod<FunctionBulder = T>
    {
        T::new(client, method_name)
    }
}
