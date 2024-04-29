use crate::*;
use serde::Serialize;
use serde_with_macros::skip_serializing_none;

impl<'a> TaskReqTrait for HCaptchaTask<'a> {}

impl<'a> TaskReqTrait for HCaptchaTaskProxyless<'a> {}

#[skip_serializing_none]
#[derive(Serialize, Default, Clone, Debug)]
#[serde(tag = "type")]
pub struct HCaptchaTask<'a> {
    pub websiteURL: &'a str,
    pub websiteKey: &'a str,
    pub isInvisible: Option<bool>,
    pub data: Option<&'a str>,

    #[serde(flatten)]
    pub proxy: ProxySettings<'a>,

    pub userAgent: Option<&'a str>,
    pub cookies: Option<&'a str>,
    pub fallbackToActualUA: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Default, Clone, Debug)]
#[serde(tag = "type")]
pub struct HCaptchaTaskProxyless<'a> {
    pub websiteURL: &'a str,
    pub websiteKey: &'a str,
    pub isInvisible: Option<bool>,
    pub data: Option<&'a str>,
    pub userAgent: Option<&'a str>,
    pub cookies: Option<&'a str>,
    pub fallbackToActualUA: Option<bool>,
}
