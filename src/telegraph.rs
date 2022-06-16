use std::collections::HashMap;
use reqwest::{blocking::Client, Error};

use crate::types::{TelegraphResult, Account};


struct MethodName {
    create_account: &'static str
}

impl Default for MethodName{
    fn default() -> Self {
        MethodName {
            create_account: "https://api.telegra.ph/createAccount"
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

    pub fn create_account(&self, short_name: &str, author_name: Option<&str>, author_url: Option<&str>) -> Result<Account, Error>{
        let params = HashMap::from([
            ("short_name", short_name),
            ("author_name", author_name.unwrap_or_default()),
            ("author_url", author_url.unwrap_or_default()),
        ]);
        let req = self.client.post(self.method_name.create_account).form(&params).send()?;
        let json: TelegraphResult<Account> = req.json()?;
        // TODO: Handle error if ok false or result None
        Ok(json.result.unwrap_or_default())
    }
}
