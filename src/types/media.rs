use serde::Deserialize;


#[cfg(feature = "upload")]
#[allow(missing_docs)]
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum UploadResult {
    Error { error: String },
    Ok(Vec<Media>),
}


#[cfg(feature = "upload")]
/// Media file
#[derive(Debug, Deserialize)]
pub struct Media {
    /// Path of the file uploaded.
    pub src: String,
}
