use reqwest::{blocking::Client, Error};
use serde::{Serialize, Serializer};
use serde::ser;
use serde_json;

use crate::types::{Node, Page, TelegraphResult};


#[derive(Default, Serialize)]
struct CreatePageInner {
    access_token: String,
    title: String,
    #[serde(serialize_with = "CreatePageInner::serialize")]
    content: Vec<Node>,
    author_name: Option<String>,
    author_url: Option<String>,
    return_content: bool
}

impl CreatePageInner {
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

pub struct CreatePage<'client> {
    client: &'client Client,
    method_name: &'static str,
    inner: CreatePageInner
}



impl<'client> CreatePage<'client> {
    pub fn new(client: &'client Client, method_name: &'static str) -> Self {
        CreatePage { client, method_name, inner: CreatePageInner::default() }
    }

    pub fn access_token(&mut self, access_token: &str) -> &mut Self {
        self.inner.access_token = access_token.into();
        self
    }

    pub fn title(&mut self, title: &str) -> &mut Self {
        self.inner.title = title.into();
        self
    }

    pub fn content(&mut self, content: Vec<Node>) -> &mut Self {
        self.inner.content = content;
        self
    }

    pub fn author_name(&mut self, author_name: &str) -> &mut Self {
        self.inner.author_name = Some(author_name.into());
        self
    }

    pub fn author_url(&mut self, author_url: &str) -> &mut Self {
        self.inner.author_url = Some(author_url.into());
        self
    }

    pub fn return_content(&mut self, return_content: bool) -> &mut Self {
        self.inner.return_content = return_content;
        self
    }

    // TODO: use trait for send request
    pub fn send(&self) -> Result<Page, Error> {
        let req = Client::new().post(self.method_name).form(&self.inner).send()?;
        let json: TelegraphResult<Page> = req.json()?;
        Ok(json.result.unwrap_or_default())
    }

}
