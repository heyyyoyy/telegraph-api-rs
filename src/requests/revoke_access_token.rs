use std::rc::Rc;

use reqwest::{blocking::Client, Error};
use serde::Serialize;

use crate::{types::{Account, TelegraphResult}, ApiMethod};


#[derive(Default, Serialize)]
pub struct RevokeAccessToken {
    #[serde(skip)]
    client: Rc<Client>,
    #[serde(skip)]
    method_name: Rc<String>,

    access_token: String
}


impl ApiMethod for RevokeAccessToken {
    type FunctionBulder = RevokeAccessToken;
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


impl RevokeAccessToken {
    pub fn access_token(&mut self, access_token: &str) -> &mut Self {
        self.access_token = access_token.into();
        self
    }
}
