use std::rc::Rc;

use reqwest::{blocking::Client, Error};
use serde::Serialize;

use crate::{types::{PageList, TelegraphResult}, ApiMethod};


#[derive(Serialize)]
pub struct GetPageList {
    #[serde(skip)]
    client: Rc<Client>,
    #[serde(skip)]
    method_name: Rc<String>,

    access_token: String,
    offset: i32,
    limit: i32
}

impl ApiMethod for GetPageList {
    type FunctionBulder = GetPageList;
    type Response = PageList;

    fn new(client: Rc<Client>, method_name: Rc<String>) -> Self::FunctionBulder {
        Self::FunctionBulder { 
            client, 
            method_name, 
            access_token: "".to_string(),
            offset: 0,
            limit: 50 
        }
    }
    fn send(&self) -> Result<Self::Response, Error> {
        let req = self.client.post(self.method_name.as_str()).form(&self).send()?;
        let json: TelegraphResult<Self::Response> = req.json()?;
        Ok(json.result.unwrap_or_default())
    }
}


impl GetPageList {
    pub fn access_token(&mut self, access_token: &str) -> &mut Self {
        self.access_token = access_token.into();
        self
    }

    pub fn offset(&mut self, offset: i32) -> &mut Self {
        self.offset = offset;
        self
    }

    pub fn limit(&mut self, limit: i32) -> &mut Self {
        self.limit = limit;
        self
    }
}
