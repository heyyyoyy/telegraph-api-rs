use std::rc::Rc;

use reqwest::blocking::Client;
use serde::Serialize;


use crate::error::TelegraphError;
use crate::types::{AccountField, TelegraphResult, Account};
use crate::requests::{ApiFieldSerializer, NoAccessToken, AccessToken};


/// Builder of `getAccountInfo`
#[derive(Serialize)]
pub struct GetAccountInfo<T> {
    #[serde(skip)]
    client: Rc<Client>,
    #[serde(skip)]
    method_name: Rc<String>,

    access_token: T,
    #[serde(serialize_with = "ApiFieldSerializer::serialize")]
    fields: Option<Vec<AccountField>>
}

impl GetAccountInfo<NoAccessToken> {
    pub(crate) fn new(client: Rc<Client>, method_name: Rc<String>) -> GetAccountInfo<NoAccessToken> {
        Self { 
            client, 
            method_name, 
            access_token: NoAccessToken, 
            fields: vec![AccountField::ShortName, AccountField::AuthorName, AccountField::AuthorUrl].into()
        }
    }
}

impl<T> GetAccountInfo<T> {
    /// Required. Access token of the Telegraph account.
    pub fn access_token(self, access_token: &str) -> GetAccountInfo<AccessToken> {
        GetAccountInfo { 
            client: self.client, 
            method_name: self.method_name, 
            access_token: AccessToken(access_token.into()), 
            fields: self.fields
        }
    }

    /// List of account fields to return. 
    /// Available fields [`AccountField`].
    pub fn fields(mut self, fields: Vec<AccountField>) -> Self {
        self.fields = fields.into();
        self
    }
}

impl GetAccountInfo<AccessToken> {
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
