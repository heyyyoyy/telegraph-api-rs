use std::rc::Rc;

use reqwest::blocking::Client;
use serde::Serialize;

use crate::error::TelegraphError;
use crate::requests::{
    ApiFieldSerializer, NoAccessToken, AccessToken,
    NoTitle, Title, NoContent, Content, NoPath, Path
};
use crate::types::{Node, Page, TelegraphResult};


/// Builder of `editPage`
#[derive(Default, Serialize)]
pub struct EditPage<A, P, T, C> 
where C: Serialize
{
    #[serde(skip)]
    client: Rc<Client>,
    #[serde(skip)]
    method_name: Rc<String>,

    access_token: A,
    path: P,
    title: T,
    #[serde(serialize_with = "ApiFieldSerializer::serialize")]
    content: C,
    #[serde(skip_serializing_if = "Option::is_none")]
    author_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    author_url: Option<String>,
    return_content: bool
}


impl EditPage<NoAccessToken, NoPath, NoTitle, NoContent> {
    pub(crate) fn new(client: Rc<Client>, method_name: Rc<String>) -> EditPage<NoAccessToken, NoPath, NoTitle, NoContent> {
        Self {  client, method_name, ..Self::default() }
    }
}


impl<A, P, T, C: Serialize> EditPage<A, P, T, C> {
    /// Required. Access token of the Telegraph account.
    pub fn access_token(self, access_token: &str) -> EditPage<AccessToken, P, T, C> {
        EditPage { 
            client: self.client, 
            method_name: self.method_name,  
            access_token: AccessToken(access_token.into()),  
            path: self.path, 
            title: self.title, 
            content: self.content, 
            author_name: self.author_name, 
            author_url: self.author_url, 
            return_content: self.return_content 
        }
    }

    /// Required. Path to the page.
    pub fn title(self, title: &str) -> EditPage<A, P, Title, C> {
        EditPage { 
            client: self.client, 
            method_name: self.method_name,  
            access_token: self.access_token,  
            path: self.path, 
            title: Title(title.into()), 
            content: self.content, 
            author_name: self.author_name, 
            author_url: self.author_url, 
            return_content: self.return_content 
        }
    }

    /// Required. Page title.
    pub fn path(self, path: &str) -> EditPage<A, Path, T, C> {
        EditPage { 
            client: self.client, 
            method_name: self.method_name,  
            access_token: self.access_token,  
            path: Path(path.into()), 
            title: self.title, 
            content: self.content, 
            author_name: self.author_name, 
            author_url: self.author_url, 
            return_content: self.return_content 
        }
    }

    /// Required. Content of the page.
    pub fn content(self, content: Vec<Node>) -> EditPage<A, P, T, Content> {
        EditPage { 
            client: self.client, 
            method_name: self.method_name,  
            access_token: self.access_token,  
            path: self.path, 
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

impl EditPage<AccessToken, Path, Title, Content> {
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
