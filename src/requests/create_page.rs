use std::rc::Rc;

use reqwest::blocking::Client;
use serde::Serialize;

use crate::error::TelegraphError;
use crate::requests::{ApiFieldSerializer, NoAccessToken, AccessToken};
use crate::types::{Node, Page, TelegraphResult};


#[derive(Default)]
pub struct NoTitle;

#[derive(Serialize)]
pub struct Title(String);

#[derive(Default, Serialize)]
pub struct NoContent;

#[derive(Serialize)]
pub struct Content(Vec<Node>);

/// Builder of `createPage`
#[derive(Default, Serialize)]
pub struct CreatePage<A, T, C> 
where C: Serialize
{
    #[serde(skip)]
    client: Rc<Client>,
    #[serde(skip)]
    method_name: Rc<String>,

    access_token: A,
    title: T,
    #[serde(serialize_with = "ApiFieldSerializer::serialize")]
    content: C,
    #[serde(skip_serializing_if = "Option::is_none")]
    author_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    author_url: Option<String>,
    return_content: bool
}

impl CreatePage<NoAccessToken, NoTitle, NoContent> {
    pub(crate) fn new(client: Rc<Client>, method_name: Rc<String>) -> CreatePage<NoAccessToken, NoTitle, NoContent> {
        Self {  client, method_name, ..Self::default() }
    }
}

impl<A, T, C: Serialize> CreatePage<A, T, C> {
    /// Required. Access token of the Telegraph account.
    pub fn access_token(self, access_token: &str) -> CreatePage<AccessToken, T, C> {
        CreatePage { 
            client: self.client, 
            method_name: self.method_name, 
            access_token: AccessToken(access_token.into()), 
            title: self.title, 
            content: self.content, 
            author_name: self.author_name, 
            author_url: self.author_url, 
            return_content: self.return_content 
        }
    }

    /// Required. Page title.
    pub fn title(self, title: &str) -> CreatePage<A, Title, C> {
        CreatePage { 
            client: self.client, 
            method_name: self.method_name, 
            access_token: self.access_token, 
            title: Title(title.into()), 
            content: self.content, 
            author_name: self.author_name, 
            author_url: self.author_url, 
            return_content: self.return_content 
        }
    }

    /// Required. Content of the page.
    pub fn content(self, content: Vec<Node>) -> CreatePage<A, T, Content> {
        CreatePage { 
            client: self.client, 
            method_name: self.method_name, 
            access_token: self.access_token, 
            title: self.title, 
            content: Content(content), 
            author_name: self.author_name, 
            author_url: self.author_url, 
            return_content: self.return_content 
        }
    }

    /// Author name, displayed below the article's title.
    pub fn author_name(mut self, author_name: &str) -> Self {
        self.author_name = Some(author_name.into());
        self
    }

    /// Profile link, opened when users click on the author's name 
    /// below the title. Can be any link, not necessarily to a Telegram profile or channel.
    pub fn author_url(mut self, author_url: &str) -> Self {
        self.author_url = Some(author_url.into());
        self
    }

    /// If `true`, a content field will be returned in the [`Page`].
    pub fn return_content(mut self, return_content: bool) -> Self {
        self.return_content = return_content;
        self
    }
}

impl CreatePage<AccessToken, Title, Content> {
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
