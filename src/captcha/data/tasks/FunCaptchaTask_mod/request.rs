use crate::{ProxySettings, TaskReqTrait};
use serde::Serialize;
use serde_with_macros::skip_serializing_none;

impl<'a> TaskReqTrait for FunCaptchaTask<'a> {}

impl<'a> TaskReqTrait for FunCaptchaTaskProxyless<'a> {}

#[skip_serializing_none]
#[derive(Serialize, Default, Clone, Debug)]
#[serde(tag = "type")]
pub struct FunCaptchaTask<'a> {
    pub websiteURL: &'a str,
    pub funcaptchaApiJSSubdomain: Option<&'a str>,
    pub websitePublicKey: &'a str,
    pub data: Option<&'a str>,

    #[serde(flatten)]
    pub proxy: ProxySettings<'a>,

    pub userAgent: Option<&'a str>,
    pub cookies: Option<&'a str>,
}

#[skip_serializing_none]
#[derive(Serialize, Default, Clone, Debug)]
#[serde(tag = "type")]
pub struct FunCaptchaTaskProxyless<'a> {
    pub websiteURL: &'a str,
    pub funcaptchaApiJSSubdomain: Option<&'a str>,
    pub websitePublicKey: &'a str,
    pub data: Option<&'a str>,
}
