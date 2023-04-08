use std::rc::Rc;

use reqwest::blocking::Client;
use serde::Serialize;

use crate::types::{PageList, TelegraphResult};
use crate::requests::Request;
use crate::error::TelegraphError;


/// Builder of `getPageList`
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

impl Request for GetPageList {
    type MethodBuilder = GetPageList;
    type Response = PageList;

    fn new(client: Rc<Client>, method_name: Rc<String>) -> Self::MethodBuilder {
        Self::MethodBuilder { 
            client, 
            method_name, 
            access_token: "".to_string(),
            offset: 0,
            limit: 50 
        }
    }
    fn send(&self) -> Result<Self::Response, TelegraphError> {
        let req = self.client.post(self.method_name.as_str()).form(&self).send()?;
        let json: TelegraphResult<Self::Response> = req.json()?;
        Self::MethodBuilder::catch_api_error(json)
    }
}


impl GetPageList {
    /// Required. Access token of the Telegraph account.
    pub fn access_token(&mut self, access_token: &str) -> &mut Self {
        self.access_token = access_token.into();
        self
    }

    /// Sequential number of the first page to be returned.
    pub fn offset(&mut self, offset: i32) -> &mut Self {
        self.offset = offset;
        self
    }

    /// Limits the number of pages to be retrieved.
    pub fn limit(&mut self, limit: i32) -> &mut Self {
        self.limit = limit;
        self
    }
}
