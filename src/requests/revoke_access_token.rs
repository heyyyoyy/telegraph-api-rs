use reqwest::{blocking::Client, Error};
use serde::Serialize;

use crate::types::{Account, TelegraphResult};


#[derive(Default, Serialize)]
struct RevokeAccessTokenInner {
    access_token: String
}


pub struct RevokeAccessToken<'client> {
    client: &'client Client,
    method_name: &'static str,
    inner: RevokeAccessTokenInner
}


impl<'client> RevokeAccessToken<'client> {
    pub fn new(client: &'client Client, method_name: &'static str) -> Self {
        RevokeAccessToken { client, method_name, inner: RevokeAccessTokenInner::default() }
    }

    pub fn access_token(&mut self, access_token: &str) -> &mut Self {
        self.inner.access_token = access_token.into();
        self
    }

    // TODO: use trait for send request
    pub fn send(&self) -> Result<Account, Error> {
        let req = Client::new().post(self.method_name).form(&self.inner).send()?;
        let json: TelegraphResult<Account> = req.json()?;
        Ok(json.result.unwrap_or_default())
    }
}