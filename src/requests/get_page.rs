use std::rc::Rc;

use reqwest::{blocking::Client, Error};
use serde::Serialize;

use crate::types::{Page, TelegraphResult};
use crate::Request;


#[derive(Default, Serialize)]
pub struct GetPage {
    #[serde(skip)]
    client: Rc<Client>,
    #[serde(skip)]
    method_name: Rc<String>,

    path: String,
    return_content: bool
}

impl Request for GetPage {
    type MethodBuilder = GetPage;
    type Response = Page;

    fn new(client: Rc<Client>, method_name: Rc<String>) -> Self::MethodBuilder {
        Self::MethodBuilder { client, method_name, ..Self::default() }
    }
    fn send(&self) -> Result<Self::Response, Error> {
        let req = self.client.post(self.method_name.as_str()).form(&self).send()?;
        let json: TelegraphResult<Self::Response> = req.json()?;
        Ok(json.result.unwrap_or_default())
    }
}


impl GetPage {
    pub fn path(&mut self, path: &str) -> &mut Self {
        self.path = path.into();
        self
    }

    pub fn return_content(&mut self, return_content: bool) -> &mut Self {
        self.return_content = return_content;
        self
    }
}
