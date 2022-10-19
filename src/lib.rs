#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]
//! [Telegraph API](https://telegra.ph/api) lib in Rust
//! 
//! # Quick start
//! 
//! ```rust
//! use telegraph_rs::{Telegraph, Request};
//! 
//! let telegraph = Telegraph::new();
//! let account = telegraph.create_account()
//! .short_name("Short name")
//! .author_name("Author name")
//! .send()
//! .unwrap();
//! ```
//! [`Telegraph`] contains methods Telegraph API 
//! that implemented trait [`Request`][`crate::requests::Request`] 
//! and returns [`MethodBuilder`][`crate::requests::Request::MethodBuilder`].
//! Fill the required parameters and use method `send`.  
//! 
//! # Errors
//! 
//! Possible errors are described in [`TelegraphError`].

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


/// `Telegraph` for calling method builder
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
    /// Constructs a new `Telegraph`
    pub fn new() -> Self {
        Telegraph::default()
    }

    /// Use this method to create a new Telegraph [`Account`][crate::types::Account]. 
    /// Most users only need one account, but this can be useful 
    /// for channel administrators who would like to keep individual 
    /// author names and profile links for each of their channels. 
    /// On success, returns an [`Account`][crate::types::Account] with the regular 
    /// fields and an additional `access_token` field.
    /// 
    /// # Example
    /// ```rust, no_run
    /// use telegraph_rs::{Telegraph, Request};
    ///
    /// let telegraph = Telegraph::new();
    /// let account = telegraph.create_account()
    /// .short_name("Short name")
    /// .author_name("Author name")
    /// .send()
    /// .unwrap();
    /// ```
    pub fn create_account(&self) -> CreateAccount {
        RequestBuilder::build::<CreateAccount>(
            self.client.clone(), 
            self.method_name.create_account.clone()
        )
    }

    /// Use this method to update information about a Telegraph account. 
    /// Pass only the parameters that you want to edit. 
    /// On success, returns an [`Account`][crate::types::Account] 
    /// with the default fields.
    /// 
    /// # Example
    /// ```rust, no_run
    /// use telegraph_rs::{Telegraph, Request};
    ///
    /// let telegraph = Telegraph::new();
    /// 
    /// let token = "1234"; // Token from create method
    /// let account = telegraph.edit_account_info()
    /// .access_token(token)
    /// .author_name("Test")
    /// .send()
    /// .unwrap();
    /// ```
    pub fn edit_account_info(&self) -> EditAccountInfo 
    {
        RequestBuilder::build::<EditAccountInfo>(
            self.client.clone(), 
            self.method_name.edit_account_info.clone()
        )
    }

    /// Use this method to get information about a Telegraph account. 
    /// Returns an [`Account`][crate::types::Account] on success.
    /// 
    /// # Example
    /// ```rust, no_run
    /// use telegraph_rs::{Telegraph, Request, types::AccountField};
    /// 
    /// let telegraph = Telegraph::new();
    /// 
    /// let token = "1234"; // Token from create method
    /// let account = telegraph.get_account_info()
    /// .access_token(token)
    /// .fields(vec![AccountField::ShortName, AccountField::AuthorUrl])
    /// .send()
    /// .unwrap();
    /// ```
    pub fn get_account_info(&self) -> GetAccountInfo {
        RequestBuilder::build::<GetAccountInfo>(
            self.client.clone(), 
            self.method_name.get_account_info.clone()
        )
    }

    /// Use this method to revoke `access_token` and generate a new one, 
    /// for example, if the user would like to reset all connected sessions, 
    /// or you have reasons to believe the token was compromised. 
    /// On success, returns an [`Account`][crate::types::Account] with 
    /// new `access_token` and `auth_url` fields.
    /// 
    /// # Example
    /// ```rust, no_run
    /// use telegraph_rs::{Telegraph, Request, types::AccountField};
    /// 
    /// let telegraph = Telegraph::new();
    /// 
    /// let token = "1234"; // Token from create method
    /// let account = telegraph.revoke_access_token()
    /// .access_token(token)
    /// .send()
    /// .unwrap();
    /// ```
    pub fn revoke_access_token(&self) -> RevokeAccessToken {
        RequestBuilder::build::<RevokeAccessToken>(
            self.client.clone(), 
            self.method_name.revoke_access_token.clone()
        )
    }

    /// Use this method to create a new Telegraph page. 
    /// On success, returns a [`Page`][crate::types::Page].
    ///
    /// # Example
    /// ```rust, no_run
    /// use telegraph_rs::{Telegraph, Request, build_content, types::AccountField};
    /// 
    /// let telegraph = Telegraph::new();
    /// 
    /// let token = "1234"; // Token from create method
    /// let content = r#"
    /// [
    ///     {
    ///         "tag": "h3",
    ///         "children": ["Hello telegraph-rs"]
    ///     },
    ///     {
    ///         "tag": "h4",
    ///         "children": ["title"]
    ///     },
    ///     {
    ///         "tag": "p",
    ///         "children": [
    ///             {
    ///                 "tag": "ul",
    ///                 "children": ["Some text"]
    ///             }
    ///         ]
    ///     }
    /// ]
    /// "#;
    ///
    /// let cont = build_content(content).unwrap();
    ///
    /// let page = telegraph.create_page()
    /// .access_token(token)
    /// .title("Hello world")
    /// .content(cont)
    /// .return_content(true)
    /// .send()
    /// .unwrap();
    /// ```
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
