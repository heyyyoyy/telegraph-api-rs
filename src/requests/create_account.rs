use serde::Serialize;

#[derive(Serialize)]
pub struct CreateAccount {
    short_name: String,
    author_name: Option<String>,
    author_url: Option<String>
}


impl<'create_account> CreateAccount {
    pub fn new(
        short_name: &'create_account str,
        author_name: impl Into<Option<&'create_account str>>,
        author_url: impl Into<Option<&'create_account str>>
    ) -> Self
    {
        CreateAccount {
            short_name: short_name.into(),
            author_name: String::from(author_name.into().unwrap_or("")).into(), 
            author_url: String::from(author_url.into().unwrap_or("")).into()
        }
    }
}
