use std::rc::Rc;

use reqwest::blocking::Client;
use serde::Serialize;

use crate::types::{Account, TelegraphResult};
use crate::error::TelegraphError;
use crate::requests::{NoAccessToken, AccessToken};


/// Builder of `editAccountInfo`
#[derive(Default, Serialize)]
pub struct EditAccountInfo<T> {
    #[serde(skip)]
    client: Rc<Client>,
    #[serde(skip)]
    method_name: Rc<String>,

    access_token: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    short_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    author_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    author_url: Option<String>
}


impl EditAccountInfo<NoAccessToken> {
    pub(crate) fn new(client: Rc<Client>, method_name: Rc<String>) -> EditAccountInfo<NoAccessToken> {
        EditAccountInfo {  client, method_name, ..Self::default() }
    }
}


impl<T> EditAccountInfo<T> {
    /// Required. Access token of the Telegraph account.
    pub fn access_token(self, access_token: &str) -> EditAccountInfo<AccessToken> {
        EditAccountInfo { 
            client: self.client, 
            method_name: self.method_name, 
            access_token: AccessToken(access_token.into()), 
            short_name: self.short_name, 
            author_name: self.author_name, 
            author_url: self.author_url
        }
    }

    /// New account name.
    pub fn short_name(mut self, short_name: &str) -> Self {
        self.short_name = Some(short_name.into());
        self
    }

    /// New default author name used when creating new articles.
    pub fn author_name(mut self, author_name: &str) -> Self {
        self.author_name = Some(author_name.into());
        self
    }

    /// New default profile link, opened when users click on the author's 
    /// name below the title. Can be any link, 
    /// not necessarily to a Telegram profile or channel.
    pub fn author_url(mut self, author_url: &str) -> Self {
        self.author_url = Some(author_url.into());
        self
    }
}

impl EditAccountInfo<AccessToken> {
    /// Sending request to API
    pub fn send(self) -> Result<Account, TelegraphError> {
        let req = self.client.post(self.method_name.as_str()).form(&self).send()?;
        let json: TelegraphResult<Account> = req.json()?;
        if !json.ok {
            Err(json.error.unwrap())
        } else {
            Ok(json.result.unwrap())
        }
    }
}
