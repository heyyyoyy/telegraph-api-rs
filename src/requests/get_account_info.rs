use std::sync::Arc;

use reqwest::{blocking::Client, Error};
use serde::{Serialize, Serializer};
use serde::ser;


use crate::Request;
use crate::types::{AccountField, TelegraphResult, Account};


#[derive(Serialize)]
pub struct GetAccountInfo {
    #[serde(skip)]
    client: Arc<Client>,
    #[serde(skip)]
    method_name: Arc<String>,

    access_token: String,
    #[serde(serialize_with = "GetAccountInfo::serialize")]
    fields: Option<Vec<AccountField>>
}


impl GetAccountInfo {
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


impl Request for GetAccountInfo {
    type MethodBuilder = GetAccountInfo;
    type Response = Account;

    fn new(client: Arc<Client>, method_name: Arc<String>) -> Self::MethodBuilder {
        Self::MethodBuilder { 
            client, 
            method_name, 
            access_token: "".into(), 
            fields: vec![AccountField::ShortName, AccountField::AuthorName, AccountField::AuthorUrl].into()}
    }
    fn send(&self) -> Result<Self::Response, Error> {
        let req = self.client.post(self.method_name.as_str()).form(&self).send()?;
        let json: TelegraphResult<Self::Response> = req.json()?;
        Ok(json.result.unwrap_or_default())
    }
}




impl GetAccountInfo {
    pub fn access_token(&mut self, access_token: &str) -> &mut Self {
        self.access_token = access_token.into();
        self
    }

    pub fn fields(&mut self, fields: Vec<AccountField>) -> &mut Self {
        self.fields = fields.into();
        self
    }
}
