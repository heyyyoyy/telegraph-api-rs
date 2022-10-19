use std::sync::Arc;

use reqwest::blocking::Client;
use serde::Serialize;

use crate::{types::{Account, TelegraphResult}, Request, TelegraphError};


/// Builder of `createAccount`
#[derive(Default, Serialize)]
pub struct CreateAccount {
    #[serde(skip)]
    client: Arc<Client>,
    #[serde(skip)]
    method_name: Arc<String>,

    short_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    author_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    author_url: Option<String>
}


impl Request for CreateAccount {
    type MethodBuilder = CreateAccount;
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


impl CreateAccount {
    /// Settting short_name
    pub fn short_name(&mut self, short_name: &str) -> &mut Self {
        self.short_name = short_name.into();
        self
    }

    /// Settting author_name
    pub fn author_name(&mut self, author_name: &str) -> &mut Self {
        self.author_name = Some(author_name.into());
        self
    }

    /// Settting author_url
    pub fn author_url(&mut self, author_url: &str) -> &mut Self {
        self.author_url = Some(author_url.into());
        self
    }
}
