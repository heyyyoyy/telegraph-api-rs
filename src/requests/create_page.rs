use std::rc::Rc;

use reqwest::blocking::Client;
use serde::Serialize;

use crate::error::TelegraphError;
use crate::requests::{Request, ApiFieldSerializer};
use crate::types::{Node, Page, TelegraphResult};


/// Builder of `createPage`
#[derive(Default, Serialize)]
pub struct CreatePage {
    #[serde(skip)]
    client: Rc<Client>,
    #[serde(skip)]
    method_name: Rc<String>,

    access_token: String,
    title: String,
    #[serde(serialize_with = "ApiFieldSerializer::serialize")]
    content: Vec<Node>,
    #[serde(skip_serializing_if = "Option::is_none")]
    author_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    author_url: Option<String>,
    return_content: bool
}


impl Request for CreatePage {
    type MethodBuilder = CreatePage;
    type Response = Page;

    fn new(client: Rc<Client>, method_name: Rc<String>) -> Self::MethodBuilder {
        Self::MethodBuilder { client, method_name, ..Self::default() }
    }
    fn send(&self) -> Result<Self::Response, TelegraphError> {
        let req = self.client.post(self.method_name.as_str()).form(&self).send()?;
        let json: TelegraphResult<Self::Response> = req.json()?;
        Self::MethodBuilder::catch_api_error(json)
    }
}


impl CreatePage {
    /// Required. Access token of the Telegraph account.
    pub fn access_token(&mut self, access_token: &str) -> &mut Self {
        self.access_token = access_token.into();
        self
    }

    /// Required. Page title.
    pub fn title(&mut self, title: &str) -> &mut Self {
        self.title = title.into();
        self
    }

    /// Required. Content of the page.
    pub fn content(&mut self, content: Vec<Node>) -> &mut Self {
        self.content = content;
        self
    }

    /// Author name, displayed below the article's title.
    pub fn author_name(&mut self, author_name: &str) -> &mut Self {
        self.author_name = Some(author_name.into());
        self
    }

    /// Profile link, opened when users click on the author's name 
    /// below the title. Can be any link, not necessarily to a Telegram profile or channel.
    pub fn author_url(&mut self, author_url: &str) -> &mut Self {
        self.author_url = Some(author_url.into());
        self
    }

    /// If `true`, a content field will be returned in the [`Page`].
    pub fn return_content(&mut self, return_content: bool) -> &mut Self {
        self.return_content = return_content;
        self
    }
}
