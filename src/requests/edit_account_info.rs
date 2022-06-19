use serde::Serialize;

#[derive(Serialize)]
pub struct EditAccountinfo {
    access_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    short_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    author_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    author_url: Option<String>
}

impl EditAccountinfo {
    pub fn new<'edit_account>(
        access_token: &'edit_account str,
        short_name: impl Into<Option<&'edit_account str>>,
        author_name: impl Into<Option<&'edit_account str>>,
        author_url: impl Into<Option<&'edit_account str>>
    ) -> Self 
    {
        EditAccountinfo {
            access_token: access_token.into(),
            short_name: if let Some(val) = short_name.into() {Some(val.to_string())} else {None},
            author_name: if let Some(val) = author_name.into() {Some(val.to_string())} else {None}, 
            author_url: if let Some(val) = author_url.into() {Some(val.to_string())} else {None}
        }
    }
}
