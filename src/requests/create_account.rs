use std::rc::Rc;

use reqwest::blocking::Client;
use serde::Serialize;

use crate::types::{Account, TelegraphResult};
use crate::error::TelegraphError;


/// Short name of the empty state type 
#[derive(Default)]
pub struct NoShortName;

/// Short name of the filled state type
#[derive(Serialize)]
pub struct ShortName(String);

/// Builder of `createAccount`
#[derive(Default, Serialize)]
pub struct CreateAccount<N> {
    #[serde(skip)]
    client: Rc<Client>,
    #[serde(skip)]
    method_name: Rc<String>,

    short_name: N,
    #[serde(skip_serializing_if = "Option::is_none")]
    author_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    author_url: Option<String>
}


impl CreateAccount<NoShortName> {
    pub(crate) fn new(client: Rc<Client>, method_name: Rc<String>) -> CreateAccount<NoShortName> {
        CreateAccount {  client, method_name, ..Self::default() }
    }
}


impl<N> CreateAccount<N> {
    /// Required. Account name, helps users with several accounts remember 
    /// which they are currently using. Displayed to the user above the 
    /// "Edit/Publish" button on Telegra.ph, other users don't see this name.
    pub fn short_name(self, short_name: &str) -> CreateAccount<ShortName> {
        CreateAccount { 
            client: self.client,
            method_name: self.method_name,
            short_name: ShortName(short_name.into()),
            author_name: self.author_name,
            author_url: self.author_url
        }

    }

    /// Default author name used when creating new articles.
    pub fn author_name(mut self, author_name: &str) -> Self {
        self.author_name = Some(author_name.into());
        self
    }

    /// Default profile link, opened when users click on the author's 
    /// name below the title. Can be any link, not 
    /// necessarily to a Telegram profile or channel.
    pub fn author_url(mut self, author_url: &str) -> Self {
        self.author_url = Some(author_url.into());
        self
    }
}

impl CreateAccount<ShortName> {
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