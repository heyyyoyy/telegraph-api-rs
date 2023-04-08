use std::rc::Rc;

use reqwest::blocking::Client;
use serde::Serialize;

use crate::types::{PageList, TelegraphResult};
use crate::requests::{NoAccessToken, AccessToken};
use crate::error::TelegraphError;


/// Builder of `getPageList`
#[derive(Serialize)]
pub struct GetPageList<T> {
    #[serde(skip)]
    client: Rc<Client>,
    #[serde(skip)]
    method_name: Rc<String>,

    access_token: T,
    offset: i32,
    limit: i32
}

// impl Request for GetPageList {
//     type MethodBuilder = GetPageList;
//     type Response = PageList;

//     fn new(client: Rc<Client>, method_name: Rc<String>) -> Self::MethodBuilder {
//         Self::MethodBuilder { 
//             client, 
//             method_name, 
//             access_token: "".to_string(),
//             offset: 0,
//             limit: 50 
//         }
//     }
//     fn send(&self) -> Result<Self::Response, TelegraphError> {
//         let req = self.client.post(self.method_name.as_str()).form(&self).send()?;
//         let json: TelegraphResult<Self::Response> = req.json()?;
//         Self::MethodBuilder::catch_api_error(json)
//     }
// }
impl GetPageList<NoAccessToken> {
    pub(crate) fn new(client: Rc<Client>, method_name: Rc<String>) -> GetPageList<NoAccessToken> {
        Self { 
            client, 
            method_name, 
            access_token: NoAccessToken,
            offset: 0,
            limit: 50 
        }
    }
}

impl<T> GetPageList<T> {
    /// Required. Access token of the Telegraph account.
    pub fn access_token(self, access_token: &str) -> GetPageList<AccessToken> {
        GetPageList { 
            client: self.client, 
            method_name: self.method_name, 
            access_token: AccessToken(access_token.into()), 
            offset: self.offset, 
            limit: self.limit 
        }
    }

    /// Sequential number of the first page to be returned.
    pub fn offset(mut self, offset: i32) -> Self {
        self.offset = offset;
        self
    }

    /// Limits the number of pages to be retrieved.
    pub fn limit(mut self, limit: i32) -> Self {
        self.limit = limit;
        self
    }
}

impl GetPageList<AccessToken> {
    /// Sending request to API
    pub fn send(self) -> Result<PageList, TelegraphError> {
        let req = self.client.post(self.method_name.as_str()).form(&self).send()?;
        let json: TelegraphResult<PageList> = req.json()?;
        if !json.ok {
            Err(json.error.unwrap())
        } else {
            Ok(json.result.unwrap())
        }
    }
}
