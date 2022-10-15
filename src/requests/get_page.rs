use reqwest::{blocking::Client, Error};
use serde::Serialize;

use crate::types::{Page, TelegraphResult};



#[derive(Default, Serialize)]
struct GetPageInner {
    path: String,
    return_content: bool
}


pub struct GetPage<'client> {
    client: &'client Client,
    method_name: &'static str,
    inner: GetPageInner
}


impl<'client> GetPage<'client> {
    pub fn new(client: &'client Client, method_name: &'static str) -> Self {
        GetPage { client, method_name, inner: GetPageInner::default() }
    }

    pub fn path(&mut self, path: &str) -> &mut Self {
        self.inner.path = path.into();
        self
    }

    pub fn return_content(&mut self, return_content: bool) -> &mut Self {
        self.inner.return_content = return_content;
        self
    }

    // TODO: use trait for send request
    pub fn send(&self) -> Result<Page, Error> {
        let req = self.client.post(self.method_name).form(&self.inner).send()?;
        let json: TelegraphResult<Page> = req.json()?;
        Ok(json.result.unwrap_or_default())
    }
}