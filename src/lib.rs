#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]


pub mod types;
pub mod requests;
pub mod error;

use std::sync::Arc;
use reqwest::blocking::Client;
use types::Node;

use crate::requests::{
    RequestBuilder,
    CreateAccount, EditAccountInfo, GetAccountInfo, 
    CreatePage, RevokeAccessToken, EditPage, GetPage,
    GetPageList, GetViews
};
pub use crate::requests::Request;
pub use crate::error::TelegraphError;


struct MethodName {
    create_account: Arc<String>,
    edit_account_info: Arc<String>,
    get_account_info: Arc<String>,
    revoke_access_token: Arc<String>,
    create_page: Arc<String>,
    edit_page: Arc<String>,
    get_page: Arc<String>,
    get_page_list: Arc<String>,
    get_views: Arc<String>
}


impl Default for MethodName{
    fn default() -> Self {
        MethodName {
            create_account: Arc::new("https://api.telegra.ph/createAccount".to_string()),
            edit_account_info: Arc::new("https://api.telegra.ph/editAccountInfo".to_string()),
            get_account_info: Arc::new("https://api.telegra.ph/getAccountInfo".to_string()),
            revoke_access_token: Arc::new("https://api.telegra.ph/revokeAccessToken".to_string()),
            create_page: Arc::new("https://api.telegra.ph/createPage".to_string()),
            edit_page: Arc::new("https://api.telegra.ph/editPage".to_string()),
            get_page: Arc::new("https://api.telegra.ph/getPage".to_string()),
            get_page_list: Arc::new("https://api.telegra.ph/getPageList".to_string()),
            get_views: Arc::new("https://api.telegra.ph/getViews".to_string()),
        }
    }
}


pub struct Telegraph {
    client: Arc<Client>,
    method_name: MethodName
}

impl Default for Telegraph {
    fn default() -> Self {
        Telegraph {
            client: Arc::new(Client::new()),
            method_name: MethodName::default()
        }
    }
}


impl Telegraph {
    pub fn new() -> Self {
        Telegraph::default()
    }

    pub fn create_account(&self) -> CreateAccount {
        RequestBuilder::build::<CreateAccount>(
            self.client.clone(), 
            self.method_name.create_account.clone()
        )
    }

    pub fn edit_account_info(&self) -> EditAccountInfo 
    {
        RequestBuilder::build::<EditAccountInfo>(
            self.client.clone(), 
            self.method_name.edit_account_info.clone()
        )
    }

    pub fn get_account_info(&self) -> GetAccountInfo {
        RequestBuilder::build::<GetAccountInfo>(
            self.client.clone(), 
            self.method_name.get_account_info.clone()
        )
    }

    pub fn revoke_access_token(&self) -> RevokeAccessToken {
        RequestBuilder::build::<RevokeAccessToken>(
            self.client.clone(), 
            self.method_name.revoke_access_token.clone()
        )
    }

    pub fn create_page(&self) -> CreatePage {
        RequestBuilder::build::<CreatePage>(
            self.client.clone(), 
            self.method_name.create_page.clone()
        )
    }

    pub fn edit_page(&self) -> EditPage {
        RequestBuilder::build::<EditPage>(
            self.client.clone(), 
            self.method_name.edit_page.clone()
        )
    }

    pub fn get_page(&self) -> GetPage {
        RequestBuilder::build::<GetPage>(
            self.client.clone(), 
            self.method_name.get_page.clone()
        )
    }

    pub fn get_page_list(&self) -> GetPageList {
        RequestBuilder::build::<GetPageList>(
            self.client.clone(), 
            self.method_name.get_page_list.clone()
        )
    }

    pub fn get_views(&self) -> GetViews {
        RequestBuilder::build::<GetViews>(
            self.client.clone(), 
            self.method_name.get_views.clone()
        )
    }
}


pub fn build_content(content: &str) -> Result<Vec<Node>, TelegraphError> {
    serde_json::from_str(content).map_err(TelegraphError::from)
}
