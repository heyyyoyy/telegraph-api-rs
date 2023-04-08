use std::rc::Rc;

use reqwest::blocking::Client;
use serde::Serialize;

use crate::types::{Account, TelegraphResult};
use crate::requests::{NoAccessToken, AccessToken};
use crate::error::TelegraphError;


/// Builder of `revokeAccessToken`
#[derive(Default, Serialize)]
pub struct RevokeAccessToken<T> {
    #[serde(skip)]
    client: Rc<Client>,
    #[serde(skip)]
    method_name: Rc<String>,

    access_token: T
}

impl RevokeAccessToken<NoAccessToken> {
    pub(crate) fn new(client: Rc<Client>, method_name: Rc<String>) -> RevokeAccessToken<NoAccessToken> {
        Self {  client, method_name, ..Self::default() }
    }
}

impl<T> RevokeAccessToken<T> {
    /// Required. Access token of the Telegraph account.
    pub fn access_token(self, access_token: &str) -> RevokeAccessToken<AccessToken> {
        RevokeAccessToken {
            client: self.client,
            method_name: self.method_name,
            access_token: AccessToken(access_token.into())
        }
    }
}

impl RevokeAccessToken<AccessToken> {
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
