use reqwest::{blocking::Client, Error};
use serde::Serialize;

use crate::types::{Account, TelegraphResult};

#[derive(Default, Serialize)]
struct EditAccountinfoInner {
    access_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    short_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    author_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    author_url: Option<String>
}

pub struct EditAccountInfo<'client> {
    client: &'client Client,
    method_name: &'static str,
    inner: EditAccountinfoInner
}


impl<'client> EditAccountInfo<'client> {
    pub fn new(client: &'client Client, method_name: &'static str) -> Self {
        EditAccountInfo { client, method_name, inner: EditAccountinfoInner::default() }
    }

    pub fn access_token(&mut self, access_token: &str) -> &mut Self {
        self.inner.access_token = access_token.into();
        self
    }

    pub fn short_name(&mut self, short_name: &str) -> &mut Self {
        self.inner.short_name = Some(short_name.into());
        self
    }

    pub fn author_name(&mut self, author_name: &str) -> &mut Self {
        self.inner.author_name = Some(author_name.into());
        self
    }

    pub fn author_url(&mut self, author_url: &str) -> &mut Self {
        self.inner.author_url = Some(author_url.into());
        self
    }

    // TODO: use trait for send request
    pub fn send(&self) -> Result<Account, Error> {
        let req = Client::new().post(self.method_name).form(&self.inner).send()?;
        let json: TelegraphResult<Account> = req.json()?;
        Ok(json.result.unwrap_or_default())
    }
}
