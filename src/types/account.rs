use serde::Deserialize;

pub enum AccountField {
    ShortName,
    AuthorName,
    AuthorUrl,
    AuthUrl,
    PageCount
}


impl Into<&'static str> for AccountField {
    fn into(self) -> &'static str {
        match self {
            AccountField::ShortName => "short_name",
            AccountField::AuthorName => "author_name",
            AccountField::AuthorUrl => "author_url",
            AccountField::AuthUrl => "auth_url",
            AccountField::PageCount => "page_count",
        }
    }
}


#[derive(Deserialize, Default, Debug)]
pub struct Account {
    pub short_name: String,
    pub author_name: String,
    pub author_url: String,
    pub access_token: Option<String>,
    pub auth_url: Option<String>,
    pub page_count: Option<i32>
}


#[cfg(test)]
mod tests {
    use serde_json;

    use super::Account;

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
        assert_eq!(account.short_name, "Short name");
        assert_eq!(account.author_name, "Author name");
        assert_eq!(account.author_url, "");
        assert_eq!(account.access_token.unwrap_or_default(), "qwerty1234");
        assert_eq!(account.auth_url, None);
        assert_eq!(account.page_count.unwrap_or_default(), 10);
    }
}
