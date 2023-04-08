use std::rc::Rc;

use reqwest::blocking::Client;
use serde::Serialize;

use crate::types::{Page, TelegraphResult};
use crate::error::TelegraphError;
use crate::requests::{NoPath, Path};


/// Builder of `getPage`
#[derive(Default, Serialize)]
pub struct GetPage<P> {
    #[serde(skip)]
    client: Rc<Client>,
    #[serde(skip)]
    method_name: Rc<String>,

    path: P,
    return_content: bool
}

impl GetPage<NoPath> {
    pub(crate) fn new(client: Rc<Client>, method_name: Rc<String>) -> GetPage<NoPath> {
        Self { client, method_name, ..Self::default() }
    }
}

impl<P> GetPage<P> {
    /// Required. Path to the Telegraph page 
    /// (in the format Title-12-31, 
    /// i.e. everything that comes after <http://telegra.ph/>).
    pub fn path(self, path: &str) -> GetPage<Path> {
        GetPage { 
            client: self.client, 
            method_name: self.method_name, 
            path: Path(path.into()), 
            return_content: self.return_content
        }
    }

    /// If `true`, a content field will be returned in the [`Page`].
    pub fn return_content(mut self, return_content: bool) -> Self {
        self.return_content = return_content;
        self
    }
}

impl GetPage<Path> {
    /// Sending request to API
    pub fn send(self) -> Result<Page, TelegraphError> {
        let req = self.client.post(self.method_name.as_str()).form(&self).send()?;
        let json: TelegraphResult<Page> = req.json()?;
        if !json.ok {
            Err(json.error.unwrap())
        } else {
            Ok(json.result.unwrap())
        }
    }
}
