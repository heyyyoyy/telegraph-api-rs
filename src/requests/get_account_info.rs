use serde::{Serialize, Serializer};
use serde::ser::Error;
use serde_json;

use crate::types::AccountField;

#[derive(Serialize)]
pub struct GetAccountInfo {
    access_token: String,
    #[serde(serialize_with = "GetAccountInfo::serialize")]
    fields: Option<Vec<AccountField>>
}

impl GetAccountInfo {
    pub fn new<'get_account_info>(
        access_token: &'get_account_info str,
        fields: impl Into<Option<Vec<AccountField>>>
    ) -> Self {
        GetAccountInfo {
            access_token: access_token.into(),
            fields: fields.into()
        }
    }
    fn serialize<T: Serialize, S: Serializer>(
        value: &T,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    {
        match serde_json::to_string(value) {
            Ok(json) => serializer.serialize_str(&json),
            Err(_) => Err(Error::custom("Failed to serialize value to json")),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_urlencoded;

    use super::*;

    #[test]
    fn serialize_get_account_info() {
        let params = GetAccountInfo::new(
            "qwerty",
            vec![AccountField::ShortName, AccountField::AuthorName]
        );
        let query = "access_token=qwerty&fields=%5B%22short_name%22%2C%22author_name%22%5D";

        let json = serde_urlencoded::to_string(&params);
        assert_eq!(query, json.unwrap_or("".to_string()));
    }
}
