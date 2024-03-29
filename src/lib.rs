#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]
//! Rust implementation of [Telegraph API](https://telegra.ph/api)
//! 
//! # Quick start
//! 
//! ## Create account
//! ```rust, no_run
//! use telegraph_api_rs::Telegraph;
//! 
//! let telegraph = Telegraph::new();
//! let account = telegraph.create_account()
//! .short_name("Short name")
//! .author_name("Author name")
//! .send()
//! .unwrap();
//! ```
//! 
//! ## Edit account
//! ```rust, no_run
//! # use telegraph_api_rs::{Telegraph, types::Account};
//! # let telegraph = Telegraph::new();
//! # let account = Account::default();
//! # let token = account.access_token.as_ref().unwrap();
//! let edited_account = telegraph.edit_account_info()
//! .access_token(token)
//! .author_name("Author name 2")
//! .send()
//! .unwrap();
//! ```
//! 
//! ## Get account info
//! ```rust, no_run
//! # use telegraph_api_rs::{Telegraph, types::{Account, AccountField}};
//! # let telegraph = Telegraph::new();
//! # let account = Account::default();
//! # let token = account.access_token.as_ref().unwrap();
//! let account_info = telegraph.get_account_info()
//! .access_token(token)
//! .fields(vec![AccountField::ShortName, AccountField::AuthorUrl])
//! .send()
//! .unwrap();
//! ```
//! 
//! ## Revoke access token
//! ```rust, no_run
//! # use telegraph_api_rs::{Telegraph, types::{Account}};
//! # let telegraph = Telegraph::new();
//! # let account = Account::default();
//! # let token = account.access_token.as_ref().unwrap();
//! let account = telegraph.revoke_access_token()
//! .access_token(token)
//! .send()
//! .unwrap();
//! ```
//! 
//! ## Create page
//! ```rust, no_run
//! # use telegraph_api_rs::{Telegraph, build_content, types::{Account}};
//! # let telegraph = Telegraph::new();
//! # let account = Account::default();
//! # let token = account.access_token.as_ref().unwrap();
//! let content = r#"
//! [
//!     {
//!         "tag": "h3",
//!         "children": ["Hello world"]
//!     },
//!     {
//!         "tag": "h4",
//!         "children": ["Title"]
//!     },
//!     {
//!         "tag": "p",
//!         "children": [
//!             {
//!                 "tag": "ul",
//!                 "children": ["Some text"]
//!             }
//!         ]
//!     }
//! ]
//! "#;
//!
//! let cont = build_content(content).unwrap();
//!
//! let page = telegraph.create_page()
//! .access_token(token)
//! .title("Hello world")
//! .content(cont)
//! .return_content(true)
//! .send()
//! .unwrap();
//! ```
//! 
//! ## Edit page
//! ```rust, no_run
//! # use telegraph_api_rs::{Telegraph, build_content, types::{Page, Account}};
//! # let telegraph = Telegraph::new();
//! # let account = Account::default();
//! # let page = Page::default();
//! # let token = account.access_token.as_ref().unwrap();
//! let new_content = r#"
//! [
//!    {
//!        "tag": "h3",
//!        "children": ["Hello world"]
//!    },
//!    {
//!       "tag": "h4",
//!        "children": ["Title"]
//!    },
//!    {
//!        "tag": "p",
//!        "children": [
//!            {
//!               "tag": "ul",
//!                "children": ["Some text"]
//!            },
//!            {
//!                "tag": "ul",
//!                "children": ["Some text 2"]
//!            }
//!        ]
//!    }
//! ]
//! "#;
//!
//! let new_cont = build_content(new_content).unwrap();
//!
//! let edited_page = telegraph.edit_page()
//! .access_token(token)
//! .title(&page.title)
//! .path(&page.path)
//! .content(new_cont)
//! .return_content(true)
//! .send()
//! .unwrap();
//! ```
//! 
//! ## Get page
//! ```rust, no_run
//! # use telegraph_api_rs::{Telegraph, types::{Page, Account}};
//! # let telegraph = Telegraph::new();
//! # let account = Account::default();
//! # let page = Page::default();
//! let get_page = telegraph.get_page()
//! .path(&page.path)
//! .send()
//! .unwrap();
//! ```
//! 
//! ## Get page list
//! ```rust, no_run
//! # use telegraph_api_rs::{Telegraph, types::Account};
//! # let telegraph = Telegraph::new();
//! # let account = Account::default();
//! # let token = account.access_token.as_ref().unwrap();
//! let page_list = telegraph.get_page_list()
//! .access_token(token)
//! .limit(2)
//! .send()
//! .unwrap();
//! ```
//! 
//! ## Get views
//! ```rust, no_run
//! # use telegraph_api_rs::{Telegraph, types::PageList};
//! # let telegraph = Telegraph::new();
//! # let page_list = PageList::default();
//! let count = telegraph.get_views()
//! .path(&page_list.pages[0].path)
//! .year(2022)
//! .month(10)
//! .day(15)
//! .send()
//! .unwrap();
//! ```
//! [`Telegraph`] contains methods Telegraph API  
//! 
//! # Errors
//! 
//! Possible errors are described in [`TelegraphError`].

pub mod types;
pub mod requests;
pub mod error;

use std::rc::Rc;
use std::path::Path;
use std::fs::File;
use std::io::Read;

use reqwest::blocking::{Client, multipart};
use types::Node;
#[cfg(feature = "upload")]
use types::{UploadResult, Media};

use crate::requests::{
    CreateAccount, EditAccountInfo, GetAccountInfo, 
    CreatePage, RevokeAccessToken, EditPage, GetPage,
    GetPageList, GetViews, NoShortName, NoAccessToken,
    NoTitle, NoContent, NoPath
};
pub use crate::error::TelegraphError;


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


/// `Telegraph` for calling method builder
#[derive(Default)]
pub struct Telegraph {
    client: Rc<Client>,
    method_name: MethodName
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
    /// use telegraph_api_rs::Telegraph;
    ///
    /// let telegraph = Telegraph::new();
    /// let account = telegraph.create_account()
    /// .short_name("Short name")
    /// .author_name("Author name")
    /// .send()
    /// .unwrap();
    /// ```
    pub fn create_account(&self) -> CreateAccount<NoShortName> {
        CreateAccount::new(
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
    /// # use telegraph_api_rs::{Telegraph, types::Account};
    /// # let telegraph = Telegraph::new();
    /// # let account = Account::default();
    /// # let token = account.access_token.as_ref().unwrap();
    /// let edited_account = telegraph.edit_account_info()
    /// .access_token(token)
    /// .author_name("Author name 2")
    /// .send()
    /// .unwrap();
    /// ```
    pub fn edit_account_info(&self) -> EditAccountInfo<NoAccessToken>
    {
        EditAccountInfo::new(
            self.client.clone(), 
            self.method_name.edit_account_info.clone()
        )
    }

    /// Use this method to get information about a Telegraph account. 
    /// Returns an [`Account`][crate::types::Account] on success.
    /// 
    /// # Example
    /// ```rust, no_run
    /// # use telegraph_api_rs::{Telegraph, types::{Account, AccountField}};
    /// # let telegraph = Telegraph::new();
    /// # let account = Account::default();
    /// # let token = account.access_token.as_ref().unwrap();
    /// let account_info = telegraph.get_account_info()
    /// .access_token(token)
    /// .fields(vec![AccountField::ShortName, AccountField::AuthorUrl])
    /// .send()
    /// .unwrap();
    /// ```
    pub fn get_account_info(&self) -> GetAccountInfo<NoAccessToken> {
        GetAccountInfo::new(
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
    /// # use telegraph_api_rs::{Telegraph, types::{Account}};
    /// # let telegraph = Telegraph::new();
    /// # let account = Account::default();
    /// # let token = account.access_token.as_ref().unwrap();
    /// let account = telegraph.revoke_access_token()
    /// .access_token(token)
    /// .send()
    /// .unwrap();
    /// ```
    pub fn revoke_access_token(&self) -> RevokeAccessToken<NoAccessToken> {
        RevokeAccessToken::new(
            self.client.clone(), 
            self.method_name.revoke_access_token.clone()
        )
    }

    /// Use this method to create a new Telegraph page. 
    /// On success, returns a [`Page`][crate::types::Page].
    ///
    /// # Example
    /// ```rust, no_run
    /// # use telegraph_api_rs::{Telegraph, build_content, types::{Account}};
    /// # let telegraph = Telegraph::new();
    /// # let account = Account::default();
    /// # let token = account.access_token.as_ref().unwrap();
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
    /// .access_token(token)
    /// .title("Hello world")
    /// .content(cont)
    /// .return_content(true)
    /// .send()
    /// .unwrap();
    /// ```
    pub fn create_page(&self) -> CreatePage<NoAccessToken, NoTitle, NoContent> {
        CreatePage::new(
            self.client.clone(), 
            self.method_name.create_page.clone()
        )
    }

    /// Use this method to edit an existing Telegraph page. 
    /// On success, returns a [`Page`][crate::types::Page].
    /// 
    /// # Example
    /// ```rust, no_run
    /// # use telegraph_api_rs::{Telegraph, build_content, types::{Page, Account}};
    /// # let telegraph = Telegraph::new();
    /// # let account = Account::default();
    /// # let page = Page::default();
    /// # let token = account.access_token.as_ref().unwrap();
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
    /// .access_token(token)
    /// .title(&page.title)
    /// .path(&page.path)
    /// .content(new_cont)
    /// .return_content(true)
    /// .send()
    /// .unwrap();
    /// ```
    pub fn edit_page(&self) -> EditPage<NoAccessToken, NoPath, NoTitle, NoContent> {
        EditPage::new(
            self.client.clone(), 
            self.method_name.edit_page.clone()
        )
    }

    /// Use this method to get a Telegraph page. 
    /// Returns a [`Page`][crate::types::Page] on success.
    ///
    /// # Example
    /// ```rust, no_run
    /// # use telegraph_api_rs::{Telegraph, types::{Page, Account}};
    /// # let telegraph = Telegraph::new();
    /// # let account = Account::default();
    /// # let page = Page::default();
    /// let get_page = telegraph.get_page()
    /// .path(&page.path)
    /// .send()
    /// .unwrap();
    /// ```
    pub fn get_page(&self) -> GetPage<NoPath> {
        GetPage::new(
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
    /// # use telegraph_api_rs::{Telegraph, types::Account};
    /// # let telegraph = Telegraph::new();
    /// # let account = Account::default();
    /// let page_list = telegraph.get_page_list()
    /// .access_token(&account.access_token.unwrap())
    /// .limit(2)
    /// .send()
    /// .unwrap();
    /// ```
    pub fn get_page_list(&self) -> GetPageList<NoAccessToken> {
        GetPageList::new(
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
    /// # use telegraph_api_rs::{Telegraph, types::PageList};
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
    pub fn get_views(&self) -> GetViews<NoPath> {
        GetViews::new(
            self.client.clone(), 
            self.method_name.get_views.clone()
        )
    }

    #[cfg(feature = "upload")]
    fn get_mime<T>(path: T) -> String 
    where T: AsRef<Path>
    {
        let mime = mime_guess::from_path(path).first_or(mime_guess::mime::IMAGE_JPEG);
        format!("{}/{}", mime.type_(), mime.subtype())
    }

    #[cfg(feature = "upload")]
    fn _upload<T>(client: &Client, files: &[T]) -> Result<Vec<Media>, TelegraphError> 
    where T: AsRef<Path>
    {
        let mut form = multipart::Form::new();
        for (index, file_name) in files.iter().enumerate() {
            let mut buf = vec![];
            let mut file = File::open(file_name)?;
            file.read_to_end(&mut buf)?;
            let part = multipart::Part::bytes(buf)
                .file_name(index.to_string())
                .mime_str(&Self::get_mime(file_name))?;
            form = form.part(index.to_string(), part);
        }

        let response = client.post("https://telegra.ph/upload")
        .multipart(form)
        .send()?;
        
        match response.json::<UploadResult>()? {
            UploadResult::Error { error } => Err(TelegraphError::ApiError(error)),
            UploadResult::Ok(vec) => Ok(vec)
        }
    }

    #[cfg(feature = "upload")]
    /// Upload files to telegraph
    /// 
    /// # Example
    /// ``` rust, no_run
    /// # use telegraph_api_rs::Telegraph;
    /// let telegraph = Telegraph::new();
    /// let files = vec!["1.jpg", "2.png"];
    /// let media = telegraph.upload(&files);
    /// ```
    pub fn upload<T>(&self, files: &[T]) -> Result<Vec<Media>, TelegraphError> 
    where T: AsRef<Path>
    {
        Self::_upload(&self.client, files)
    }

    #[cfg(feature = "upload")]
    /// Upload files to telegraph with custom client
    /// 
    /// # Example
    /// ``` rust, no_run
    /// # use telegraph_api_rs::Telegraph;
    /// use reqwest::blocking::Client;
    /// 
    /// let files = vec!["1.jpg", "2.png"];
    /// let client = Client::new();
    /// let media = Telegraph::upload_with(&client, &files);
    /// ```
    pub fn upload_with<T>(client: &Client, files: &[T]) -> Result<Vec<Media>, TelegraphError> 
    where T: AsRef<Path>
    {
        Self::_upload(client, files)
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
