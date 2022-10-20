#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]
//! Rust implementation of [Telegraph API](https://telegra.ph/api)
//! 
//! # Quick start
//! 
//! ```rust, no_run
//! use telegraph_api_rs::{Telegraph, Request};
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
    /// use telegraph_api_rs::{Telegraph, Request};
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
    /// # use telegraph_api_rs::{Telegraph, Request, types::Account};
    /// # let telegraph = Telegraph::new();
    /// # let account = Account::default();
    /// let edited_account = telegraph.edit_account_info()
    /// .access_token(&account.access_token.unwrap())
    /// .author_name("Author name 2")
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
    /// # use telegraph_api_rs::{Telegraph, Request, types::{Account, AccountField}};
    /// # let telegraph = Telegraph::new();
    /// # let account = Account::default();
    /// let account_info = telegraph.get_account_info()
    /// .access_token(&account.access_token.unwrap())
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
    /// # use telegraph_api_rs::{Telegraph, Request, types::{Account}};
    /// # let telegraph = Telegraph::new();
    /// # let account = Account::default();
    /// let account = telegraph.revoke_access_token()
    /// .access_token(&account.access_token.unwrap())
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
    /// # use telegraph_api_rs::{Telegraph, Request, build_content, types::{Account}};
    /// # let telegraph = Telegraph::new();
    /// # let account = Account::default();
    /// let content = r#"
    /// [
    ///     {
    ///         "tag": "h3",
    ///         "children": ["Hello world"]
    ///     },
    ///     {
    ///         "tag": "h4",
    ///         "children": ["Title"]
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
    /// .access_token(&account.access_token.unwrap())
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

    /// Use this method to edit an existing Telegraph page. 
    /// On success, returns a [`Page`][crate::types::Page].
    /// 
    /// # Example
    /// ```rust, no_run
    /// # use telegraph_api_rs::{Telegraph, Request, build_content, types::{Page, Account}};
    /// # let telegraph = Telegraph::new();
    /// # let account = Account::default();
    /// # let page = Page::default();
    /// let new_content = r#"
    /// [
    ///    {
    ///        "tag": "h3",
    ///        "children": ["Hello world"]
    ///    },
    ///    {
    ///       "tag": "h4",
    ///        "children": ["Title"]
    ///    },
    ///    {
    ///        "tag": "p",
    ///        "children": [
    ///            {
    ///               "tag": "ul",
    ///                "children": ["Some text"]
    ///            },
    ///            {
    ///                "tag": "ul",
    ///                "children": ["Some text 2"]
    ///            }
    ///        ]
    ///    }
    /// ]
    /// "#;
    ///
    /// let new_cont = build_content(new_content).unwrap();
    ///
    /// let edited_page = telegraph.edit_page()
    /// .access_token(&account.access_token.unwrap())
    /// .title(&page.title)
    /// .path(&page.path)
    /// .content(new_cont)
    /// .return_content(true)
    /// .send()
    /// .unwrap();
    /// ```
    pub fn edit_page(&self) -> EditPage {
        RequestBuilder::build::<EditPage>(
            self.client.clone(), 
            self.method_name.edit_page.clone()
        )
    }

    /// Use this method to get a Telegraph page. 
    /// Returns a [`Page`][crate::types::Page] on success.
    ///
    /// # Example
    /// ```rust, no_run
    /// # use telegraph_api_rs::{Telegraph, Request, types::{Page, Account}};
    /// # let telegraph = Telegraph::new();
    /// # let account = Account::default();
    /// # let page = Page::default();
    /// let get_page = telegraph.get_page()
    /// .path(&page.path)
    /// .send()
    /// .unwrap();
    /// ```
    pub fn get_page(&self) -> GetPage {
        RequestBuilder::build::<GetPage>(
            self.client.clone(), 
            self.method_name.get_page.clone()
        )
    }

    /// Use this method to get a list of pages belonging to a Telegraph account. 
    /// Returns a [`PageList`][crate::types::PageList], 
    /// sorted by most recently created pages first.
    ///
    /// # Example
    /// ```rust, no_run
    /// # use telegraph_api_rs::{Telegraph, Request, types::Account};
    /// # let telegraph = Telegraph::new();
    /// # let account = Account::default();
    /// let page_list = telegraph.get_page_list()
    /// .access_token(&account.access_token.unwrap())
    /// .limit(2)
    /// .send()
    /// .unwrap();
    /// ```
    pub fn get_page_list(&self) -> GetPageList {
        RequestBuilder::build::<GetPageList>(
            self.client.clone(), 
            self.method_name.get_page_list.clone()
        )
    }

    /// Use this method to get the number of views for a Telegraph article. 
    /// Returns a [`PageViews`][crate::types::PageViews] on success. 
    /// By default, the total number of page views will be returned.
    ///
    /// # Example
    /// ```rust, no_run
    /// # use telegraph_api_rs::{Telegraph, Request, types::PageList};
    /// # let telegraph = Telegraph::new();
    /// # let page_list = PageList::default();
    /// let count = telegraph.get_views()
    /// .path(&page_list.pages[0].path)
    /// .year(2022)
    /// .month(10)
    /// .day(15)
    /// .send()
    /// .unwrap();
    /// ```
    pub fn get_views(&self) -> GetViews {
        RequestBuilder::build::<GetViews>(
            self.client.clone(), 
            self.method_name.get_views.clone()
        )
    }
}


/// Build page content from string
/// 
/// # Example
/// ```rust, no_run
/// # use telegraph_api_rs::build_content;
/// let content = r#"
/// [
///    {
///        "tag": "h3",
///        "children": ["Hello world"]
///    },
///    {
///       "tag": "h4",
///        "children": ["Title"]
///    },
///    {
///        "tag": "p",
///        "children": [
///            {
///               "tag": "ul",
///                "children": ["Some text"]
///            },
///            {
///                "tag": "ul",
///                "children": ["Some text 2"]
///            }
///        ]
///    }
/// ]
/// "#;
///
/// let cont = build_content(content).unwrap();
/// ``` 
pub fn build_content(content: &str) -> Result<Vec<Node>, TelegraphError> {
    serde_json::from_str(content).map_err(TelegraphError::from)
}
