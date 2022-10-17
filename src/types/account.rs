use serde::{Deserialize, Serialize};

use super::TelegraphType;


/// Available fields of the account struct
#[derive(Serialize)]
pub enum AccountField {
    #[serde(rename = "short_name")]
    /// short_name
    ShortName,
    #[serde(rename = "author_name")]
    /// author_name
    AuthorName,
    #[serde(rename = "author_url")]
    /// author_url
    AuthorUrl,
    #[serde(rename = "auth_url")]
    /// auth_url
    AuthUrl,
    #[serde(rename = "page_count")]
    /// page_count
    PageCount
}


/// This object represents a Telegraph account
#[derive(Deserialize, Default, Debug)]
pub struct Account {
    /// Account name, helps users with several accounts remember 
    /// which they are currently using. 
    /// Displayed to the user above the "Edit/Publish" button 
    /// on Telegra.ph, other users don't see this name.
    pub short_name: Option<String>,
    /// Default author name used when creating new articles.
    pub author_name: Option<String>,
    /// Profile link, opened when users click on the author's name 
    /// below the title. Can be any link, not necessarily 
    /// to a Telegram profile or channel.
    pub author_url: Option<String>,
    /// Only returned by the `createAccount` and `revokeAccessToken` method. 
    /// Access token of the Telegraph account.
    pub access_token: Option<String>,
    /// URL to authorize a browser on telegra.ph and connect 
    /// it to a Telegraph account. This URL is valid for 
    /// only one use and for 5 minutes only.
    pub auth_url: Option<String>,
    /// Number of pages belonging to the Telegraph account.
    pub page_count: Option<i32>
}


impl TelegraphType for Account {}


#[cfg(test)]
mod tests {
    use serde_json;

    use super::{Account, AccountField};

    #[test]
    fn account_deserialize() {
        let acc_str = r#"
            {
                "short_name": "Short name",
                "author_name": "Author name",
                "author_url": "",
                "access_token": "qwerty1234",
                "page_count": 10
            }"#;
        let account: Account = serde_json::from_str(acc_str).unwrap_or_default();
        assert_eq!(account.short_name.unwrap_or_default(), "Short name");
        assert_eq!(account.author_name.unwrap_or_default(), "Author name");
        assert_eq!(account.author_url.unwrap_or_default(), "");
        assert_eq!(account.access_token.unwrap_or_default(), "qwerty1234");
        assert_eq!(account.auth_url, None);
        assert_eq!(account.page_count.unwrap_or_default(), 10);
    }

    #[test]
    fn serialize_account_fields() {
        let query = "[\"short_name\"]";
        let fields = vec![AccountField::ShortName];
        let json = serde_json::to_string(&fields);
        assert_eq!(query, json.unwrap_or("".to_string()));
    }
}
