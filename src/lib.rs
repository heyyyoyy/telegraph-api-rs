pub mod types;
pub mod requests;

use std::rc::Rc;
use reqwest::blocking::Client;
use types::Node;

use crate::requests::{
    RequestBuilder,
    CreateAccount, EditAccountInfo, GetAccountInfo, 
    CreatePage, RevokeAccessToken, EditPage, GetPage,
    GetPageList, GetViews
};
pub use crate::requests::ApiMethod;


struct MethodName {
    create_account: Rc<String>,
    edit_account_info: Rc<String>,
    get_account_info: Rc<String>,
    revoke_access_token: Rc<String>,
    create_page: Rc<String>,
    edit_page: Rc<String>,
    get_page: Rc<String>,
    get_page_list: Rc<String>,
    get_views: Rc<String>
}


impl Default for MethodName{
    fn default() -> Self {
        MethodName {
            create_account: Rc::new("https://api.telegra.ph/createAccount".to_string()),
            edit_account_info: Rc::new("https://api.telegra.ph/editAccountInfo".to_string()),
            get_account_info: Rc::new("https://api.telegra.ph/getAccountInfo".to_string()),
            revoke_access_token: Rc::new("https://api.telegra.ph/revokeAccessToken".to_string()),
            create_page: Rc::new("https://api.telegra.ph/createPage".to_string()),
            edit_page: Rc::new("https://api.telegra.ph/editPage".to_string()),
            get_page: Rc::new("https://api.telegra.ph/getPage".to_string()),
            get_page_list: Rc::new("https://api.telegra.ph/getPageList".to_string()),
            get_views: Rc::new("https://api.telegra.ph/getViews".to_string()),
        }
    }
}


pub struct Telegraph {
    client: Rc<Client>,
    method_name: MethodName
}

impl Default for Telegraph {
    fn default() -> Self {
        Telegraph {
            client: Rc::new(Client::new()),
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


pub fn build_content(content: &str) -> Result<Vec<Node>, serde_json::Error> {
    serde_json::from_str(content)
}
