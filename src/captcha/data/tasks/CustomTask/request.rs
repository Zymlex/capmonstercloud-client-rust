use serde::Serialize;
use serde_with_macros::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
#[serde(tag = "type")]
pub(crate) struct CustomTask<'a> {
    class: &'a str,
    
    #[serde(flatten)]
    data: DataDome<'a>,
}

#[skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct DataDome<'a> {
    websiteURL: &'a str,
    metadata: DataDomeMetadata<'a>,
    userAgent: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct DataDomeMetadata<'a> {
    captchaUrl: &'a str,
}