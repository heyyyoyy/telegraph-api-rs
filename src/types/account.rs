use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub enum AccountField {
    #[serde(rename = "short_name")]
    ShortName,
    #[serde(rename = "author_name")]
    AuthorName,
    #[serde(rename = "author_url")]
    AuthorUrl,
    #[serde(rename = "auth_url")]
    AuthUrl,
    #[serde(rename = "page_count")]
    PageCount
}


#[derive(Deserialize, Default, Debug)]
pub struct Account {
    pub short_name: Option<String>,
    pub author_name: Option<String>,
    pub author_url: Option<String>,
    pub access_token: Option<String>,
    pub auth_url: Option<String>,
    pub page_count: Option<i32>
}


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
