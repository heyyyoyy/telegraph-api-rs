use std::rc::Rc;

use reqwest::{blocking::Client, Error};
use serde::Serialize;

use crate::{types::{Account, TelegraphResult}, ApiMethod};


#[derive(Default, Serialize)]
pub struct CreateAccount {
    #[serde(skip)]
    client: Rc<Client>,
    #[serde(skip)]
    method_name: Rc<String>,

    short_name: String,
    author_name: Option<String>,
    author_url: Option<String>
}


impl ApiMethod for CreateAccount {
    type FunctionBulder = CreateAccount;
    type Response = Account;

    fn new(client: Rc<Client>, method_name: Rc<String>) -> Self::FunctionBulder {
        Self::FunctionBulder { client, method_name, ..Self::default() }
    }
    fn send(&self) -> Result<Self::Response, Error> {
        let req = self.client.post(self.method_name.as_str()).form(&self).send()?;
        let json: TelegraphResult<Self::Response> = req.json()?;
        Ok(json.result.unwrap_or_default())
    }
}


impl CreateAccount {
    pub fn short_name(&mut self, short_name: &str) -> &mut Self {
        self.short_name = short_name.into();
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
