use std::sync::Arc;

use reqwest::blocking::Client;
use serde::Serialize;

use crate::{types::{Account, TelegraphResult}, Request, TelegraphError};


/// Builder of `revokeAccessToken`
#[derive(Default, Serialize)]
pub struct RevokeAccessToken {
    #[serde(skip)]
    client: Arc<Client>,
    #[serde(skip)]
    method_name: Arc<String>,

    access_token: String
}


impl Request for RevokeAccessToken {
    type MethodBuilder = RevokeAccessToken;
    type Response = Account;

    fn new(client: Arc<Client>, method_name: Arc<String>) -> Self::MethodBuilder {
        Self::MethodBuilder { client, method_name, ..Self::default() }
    }
    fn send(&self) -> Result<Self::Response, TelegraphError> {
        let req = self.client.post(self.method_name.as_str()).form(&self).send()?;
        let json: TelegraphResult<Self::Response> = req.json()?;
        Self::MethodBuilder::catch_api_error(json)
    }
}


impl RevokeAccessToken {
    /// Setting access_token
    pub fn access_token(&mut self, access_token: &str) -> &mut Self {
        self.access_token = access_token.into();
        self
    }
}
