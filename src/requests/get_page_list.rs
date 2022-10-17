use std::sync::Arc;

use reqwest::{blocking::Client, Error};
use serde::Serialize;

use crate::{types::{PageList, TelegraphResult}, Request};


#[derive(Serialize)]
pub struct GetPageList {
    #[serde(skip)]
    client: Arc<Client>,
    #[serde(skip)]
    method_name: Arc<String>,

    access_token: String,
    offset: i32,
    limit: i32
}

impl Request for GetPageList {
    type MethodBuilder = GetPageList;
    type Response = PageList;

    fn new(client: Arc<Client>, method_name: Arc<String>) -> Self::MethodBuilder {
        Self::MethodBuilder { 
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
