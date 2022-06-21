use reqwest::{blocking::Client, Error};
use serde::{Serialize, Serializer};
use serde::ser;
use serde_json;

use crate::types::{Node, Page, TelegraphResult};


#[derive(Default, Serialize)]
pub struct CreatePage {
    access_token: String,
    title: String,
    #[serde(serialize_with = "CreatePage::serialize")]
    content: Vec<Node>,
    author_name: Option<String>,
    author_url: Option<String>,
    return_content: bool
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

    // TODO: use trait for send request
    pub fn send(&self) -> Result<Page, Error> {
        let req = Client::new().post("https://api.telegra.ph/createPage").form(self).send()?;
        let json: TelegraphResult<Page> = req.json()?;
        Ok(json.result.unwrap_or_default())
    }

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
