use std::rc::Rc;

use reqwest::{blocking::Client, Error};
use serde::{Serialize, Serializer};
use serde::ser;
use serde_json;

use crate::ApiMethod;
use crate::types::{Node, Page, TelegraphResult};


#[derive(Default, Serialize)]
pub struct CreatePage {
    #[serde(skip)]
    client: Rc<Client>,
    #[serde(skip)]
    method_name: Rc<String>,

    access_token: String,
    title: String,
    #[serde(serialize_with = "CreatePage::serialize")]
    content: Vec<Node>,
    author_name: Option<String>,
    author_url: Option<String>,
    return_content: bool
}


impl ApiMethod for CreatePage {
    type FunctionBulder = CreatePage;
    type Response = Page;

    fn new(client: Rc<Client>, method_name: Rc<String>) -> Self::FunctionBulder {
        Self::FunctionBulder { client, method_name, ..Self::default() }
    }
    fn send(&self) -> Result<Self::Response, Error> {
        let req = self.client.post(self.method_name.as_str()).form(&self).send()?;
        let json: TelegraphResult<Self::Response> = req.json()?;
        Ok(json.result.unwrap_or_default())
    }
}


impl CreatePage {
    // TODO: use trait with custom serialization to encode url 
    fn serialize<T: Serialize, S: Serializer>(
        value: &T,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    {
        match serde_json::to_string(value) {
            Ok(json) => serializer.serialize_str(&json),
            Err(_) => Err(ser::Error::custom("Failed to serialize value to json")),
        }
    }
}


impl CreatePage {
    pub fn access_token(&mut self, access_token: &str) -> &mut Self {
        self.access_token = access_token.into();
        self
    }

    pub fn title(&mut self, title: &str) -> &mut Self {
        self.title = title.into();
        self
    }

    pub fn content(&mut self, content: Vec<Node>) -> &mut Self {
        self.content = content;
        self
    }

    pub fn author_name(&mut self, author_name: &str) -> &mut Self {
        self.author_name = Some(author_name.into());
        self
    }

    pub fn author_url(&mut self, author_url: &str) -> &mut Self {
        self.author_url = Some(author_url.into());
        self
    }

    pub fn return_content(&mut self, return_content: bool) -> &mut Self {
        self.return_content = return_content;
        self
    }
}
