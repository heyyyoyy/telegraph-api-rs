use reqwest::{blocking::Client, Error};
use serde::Serialize;

use crate::types::{Account, TelegraphResult};


#[derive(Default, Serialize)]
struct CreateAccountInner {
    short_name: String,
    author_name: Option<String>,
    author_url: Option<String>
}

pub struct CreateAccount<'client> {
    client: &'client Client,
    method_name: &'static str,
    inner: CreateAccountInner
}


impl<'client> CreateAccount<'client> {
    pub fn new (client: &'client Client, method_name: &'static str) -> Self
    {
        CreateAccount {
            client, method_name, inner: CreateAccountInner::default()
        }
    }

    pub fn short_name(&mut self, short_name: &str) -> &mut Self {
        self.inner.short_name = short_name.into();
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
