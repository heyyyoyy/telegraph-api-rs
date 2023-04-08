use std::rc::Rc;

use reqwest::blocking::Client;
use serde::Serialize;


use crate::error::TelegraphError;
use crate::types::{AccountField, TelegraphResult, Account};
use crate::requests::{Request, ApiFieldSerializer};


/// Builder of `getAccountInfo`
#[derive(Serialize)]
pub struct GetAccountInfo {
    #[serde(skip)]
    client: Rc<Client>,
    #[serde(skip)]
    method_name: Rc<String>,

    access_token: String,
    #[serde(serialize_with = "ApiFieldSerializer::serialize")]
    fields: Option<Vec<AccountField>>
}


impl Request for GetAccountInfo {
    type MethodBuilder = GetAccountInfo;
    type Response = Account;

    fn new(client: Rc<Client>, method_name: Rc<String>) -> Self::MethodBuilder {
        Self::MethodBuilder { 
            client, 
            method_name, 
            access_token: "".into(), 
            fields: vec![AccountField::ShortName, AccountField::AuthorName, AccountField::AuthorUrl].into()}
    }
    fn send(&self) -> Result<Self::Response, TelegraphError> {
        let req = self.client.post(self.method_name.as_str()).form(&self).send()?;
        let json: TelegraphResult<Self::Response> = req.json()?;
        Self::MethodBuilder::catch_api_error(json)
    }
}




impl GetAccountInfo {
    /// Required. Access token of the Telegraph account.
    pub fn access_token(&mut self, access_token: &str) -> &mut Self {
        self.access_token = access_token.into();
        self
    }

    /// List of account fields to return. 
    /// Available fields [`AccountField`].
    pub fn fields(&mut self, fields: Vec<AccountField>) -> &mut Self {
        self.fields = fields.into();
        self
    }
}
