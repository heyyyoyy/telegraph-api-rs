use std::sync::Arc;

use reqwest::{blocking::Client, Error};
use serde::Serialize;

use crate::{types::{Account, TelegraphResult}, Request};


#[derive(Default, Serialize)]
pub struct EditAccountInfo {
    #[serde(skip)]
    client: Arc<Client>,
    #[serde(skip)]
    method_name: Arc<String>,

    access_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    short_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    author_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    author_url: Option<String>
}


impl Request for EditAccountInfo {
    type MethodBuilder = EditAccountInfo;
    type Response = Account;

    fn new(client: Arc<Client>, method_name: Arc<String>) -> Self::MethodBuilder {
        Self::MethodBuilder { client, method_name, ..Self::default() }
    }
    fn send(&self) -> Result<Self::Response, Error> {
        let req = self.client.post(self.method_name.as_str()).form(&self).send()?;
        let json: TelegraphResult<Self::Response> = req.json()?;
        Ok(json.result.unwrap_or_default())
    }
}


impl EditAccountInfo {
    pub fn access_token(&mut self, access_token: &str) -> &mut Self {
        self.access_token = access_token.into();
        self
    }

    pub fn short_name(&mut self, short_name: &str) -> &mut Self {
        self.short_name = Some(short_name.into());
        self
    }

    pub fn author_name(&mut self, author_name: &str) -> &mut Self {
        self.author_name = Some(author_name.into());
        self
    }

    pub fn author_url(&mut self, author_url: &str) -> &mut Self {
        self.author_url = Some(author_url.into());
        self
    }
}
