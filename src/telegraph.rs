use std::collections::HashMap;
use reqwest::{blocking::Client, Error};

use crate::types::{TelegraphResult, Account};


struct MethodName {
    create_account: &'static str,
    edit_account_info: &'static str
}

impl Default for MethodName{
    fn default() -> Self {
        MethodName {
            create_account: "https://api.telegra.ph/createAccount",
            edit_account_info: "https://api.telegra.ph/editAccountInfo"
        }
    }
}

pub struct Telegraph {
    client: Client,
    method_name: MethodName
}

impl Default for Telegraph {
    fn default() -> Self {
        Telegraph {
            client: Client::new(),
            method_name: Default::default()
        }
    }
}



impl Telegraph {
    pub fn new() -> Self {
        Telegraph::default()
    }

    pub fn create_account<'a>(
        &self, short_name: &'a str,
        author_name: impl Into<Option<&'a str>>,
        author_url: impl Into<Option<&'a str>>
    ) -> Result<Account, Error>
    {
        let params = HashMap::from([
            ("short_name", short_name),
            ("author_name", author_name.into().unwrap_or_default()),
            ("author_url", author_url.into().unwrap_or_default()),
        ]);
        let req = self.client.post(self.method_name.create_account).form(&params).send()?;
        let json: TelegraphResult<Account> = req.json()?;
        // TODO: Handle error if ok false or result None
        Ok(json.result.unwrap_or_default())
    }

    pub fn edit_account_info<'a>(
        &self,
        access_token: &'a str,
        short_name: impl Into<Option<&'a str>>,
        author_name: impl Into<Option<&'a str>>,
        author_url: impl Into<Option<&'a str>>
    ) -> Result<Account, Error> 
    {
        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("access_token", access_token);
        if let Some(val) = short_name.into() {
            params.insert("short_name", val);
        };
        if let Some(val) = author_name.into() {
            params.insert("author_name", val);
        };
        if let Some(val) = author_url.into() {
            params.insert("author_url", val);
        };
        let req = self.client.post(self.method_name.edit_account_info).form(&params).send()?;
        let json: TelegraphResult<Account> = req.json()?;
        // TODO: Handle error if ok false or result None
        Ok(json.result.unwrap_or_default())
    }
}
