use reqwest::blocking::Client;

use crate::requests::{CreateAccount, EditAccountInfo, GetAccountInfo, CreatePage, RevokeAccessToken};


struct MethodName {
    create_account: &'static str,
    edit_account_info: &'static str,
    get_account_info: &'static str
}

impl Default for MethodName{
    fn default() -> Self {
        MethodName {
            create_account: "https://api.telegra.ph/createAccount",
            edit_account_info: "https://api.telegra.ph/editAccountInfo",
            get_account_info: "https://api.telegra.ph/getAccountInfo"
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

    pub fn create_account(&self) -> CreateAccount {
        CreateAccount::new(&self.client, "https://api.telegra.ph/createAccount")
    }

    pub fn edit_account_info(&self) -> EditAccountInfo 
    {
        EditAccountInfo::new(&self.client, "https://api.telegra.ph/editAccountInfo")
    }

    pub fn get_account_info(&self) -> GetAccountInfo {
        GetAccountInfo::new(&self.client, "https://api.telegra.ph/getAccountInfo")
    }

    pub fn revoke_access_token(&self) -> RevokeAccessToken {
        RevokeAccessToken::new(&self.client, "https://api.telegra.ph/revokeAccessToken")
    }

    pub fn create_page(&self) -> CreatePage {
        CreatePage::new(&self.client, "https://api.telegra.ph/createPage")
    }
}
