use serde::Serialize;
use serde_with_macros::skip_serializing_none;
use crate::TaskReqTrait;

impl<'a> TaskReqTrait for CustomTask<'a> {}

#[skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
#[serde(tag = "type")]
pub(crate) struct CustomTask<'a> {
    pub(crate) class: &'a str,
    
    #[serde(flatten)]
    pub data: CustomTaskData<'a>,
}

#[skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct CustomTaskData<'a> {
    pub websiteURL: &'a str,
    pub metadata: CustomTaskMetadata<'a>,
    pub userAgent: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct CustomTaskMetadata<'a> {
    pub captchaUrl: &'a str,
}