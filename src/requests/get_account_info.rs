use reqwest::{blocking::Client, Error};
use serde::{Serialize, Serializer};
use serde::ser;


use crate::types::{AccountField, TelegraphResult, Account};


#[derive(Serialize)]
struct GetAccountInfoInner {
    access_token: String,
    #[serde(serialize_with = "GetAccountInfoInner::serialize")]
    fields: Option<Vec<AccountField>>
}


impl GetAccountInfoInner {
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


impl Default for GetAccountInfoInner {
    fn default() -> Self {
        Self { 
            access_token: "".into(), 
            fields: vec![AccountField::ShortName, AccountField::AuthorName, AccountField::AuthorUrl].into() 
        }
    }
}


pub struct GetAccountInfo<'client> {
    client: &'client Client,
    method_name: &'static str,
    inner: GetAccountInfoInner
}


impl<'client> GetAccountInfo<'client> {
    pub fn new(client: &'client Client, method_name: &'static str) -> Self {
        GetAccountInfo { client, method_name, inner: GetAccountInfoInner::default() }
    }

    pub fn access_token(&mut self, access_token: &str) -> &mut Self {
        self.inner.access_token = access_token.into();
        self
    }

    pub fn fields(&mut self, fields: Vec<AccountField>) -> &mut Self {
        self.inner.fields = fields.into();
        self
    }

    // TODO: use trait for send request
    pub fn send(&self) -> Result<Account, Error> {
        let req = Client::new().post(self.method_name).form(&self.inner).send()?;
        let json: TelegraphResult<Account> = req.json()?;
        Ok(json.result.unwrap_or_default())
    }
}
