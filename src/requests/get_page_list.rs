use reqwest::{blocking::Client, Error};
use serde::Serialize;

use crate::types::{PageList, TelegraphResult};


#[derive(Serialize)]
struct GetPageListInner {
    access_token: String,
    offset: i32,
    limit: i32
}

impl Default for GetPageListInner{
    fn default() -> Self {
        GetPageListInner {
            access_token: "".into(),
            offset: 0,
            limit: 50
        }
    }
}


pub struct GetPageList<'client> {
    client: &'client Client,
    method_name: &'static str,
    inner: GetPageListInner
}


impl<'client> GetPageList<'client> {
    pub fn new(client: &'client Client, method_name: &'static str) -> Self {
        GetPageList { client, method_name, inner: GetPageListInner::default() }
    }

    pub fn access_token(&mut self, access_token: &str) -> &mut Self {
        self.inner.access_token = access_token.into();
        self
    }

    pub fn offset(&mut self, offset: i32) -> &mut Self {
        self.inner.offset = offset;
        self
    }

    pub fn limit(&mut self, limit: i32) -> &mut Self {
        self.inner.limit = limit;
        self
    }

    // TODO: use trait for send request
    pub fn send(&self) -> Result<PageList, Error> {
        let req = self.client.post(self.method_name).form(&self.inner).send()?;
        let json: TelegraphResult<PageList> = req.json()?;
        Ok(json.result.unwrap_or_default())
    }
}