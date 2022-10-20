use std::sync::Arc;

use reqwest::blocking::Client;
use serde::Serialize;

use crate::types::{Page, TelegraphResult};
use crate::error::TelegraphError;
use crate::requests::Request;


/// Builder of `getPage`
#[derive(Default, Serialize)]
pub struct GetPage {
    #[serde(skip)]
    client: Arc<Client>,
    #[serde(skip)]
    method_name: Arc<String>,

    path: String,
    return_content: bool
}

impl Request for GetPage {
    type MethodBuilder = GetPage;
    type Response = Page;

    fn new(client: Arc<Client>, method_name: Arc<String>) -> Self::MethodBuilder {
        Self::MethodBuilder { client, method_name, ..Self::default() }
    }
    fn send(&self) -> Result<Self::Response, TelegraphError> {
        let req = self.client.post(self.method_name.as_str()).form(&self).send()?;
        let json: TelegraphResult<Self::Response> = req.json()?;
        Self::MethodBuilder::catch_api_error(json)
    }
}


impl GetPage {
    /// Required. Path to the Telegraph page 
    /// (in the format Title-12-31, 
    /// i.e. everything that comes after http://telegra.ph/).
    pub fn path(&mut self, path: &str) -> &mut Self {
        self.path = path.into();
        self
    }

    /// If `true`, a content field will be returned in the [`Page`].
    pub fn return_content(&mut self, return_content: bool) -> &mut Self {
        self.return_content = return_content;
        self
    }
}
