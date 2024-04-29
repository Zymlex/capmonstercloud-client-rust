use crate::{ProxySettings, TaskReqTrait};
use serde::Serialize;
use serde_with_macros::skip_serializing_none;

impl<'a> TaskReqTrait for TurnstileTask<'a> {}

impl<'a> TaskReqTrait for TurnstileTaskProxyless<'a> {}

#[skip_serializing_none]
#[derive(Serialize, Default, Clone, Debug)]
#[serde(tag = "type")]
pub struct TurnstileTaskProxyless<'a> {
    pub websiteURL: &'a str,
    pub websiteKey: &'a str,
    pub pageAction: Option<&'a str>,
}

#[skip_serializing_none]
#[derive(Serialize, Default, Clone, Debug)]
#[serde(tag = "type")]
pub struct TurnstileTask<'a> {
    pub websiteURL: &'a str,
    pub websiteKey: &'a str,

    #[serde(flatten)]
    pub proxy: ProxySettings<'a>,

    pub cloudflareTaskType: Option<CloudflareTaskType>,
    pub htmlPageBase64: Option<&'a str>,
    pub userAgent: Option<&'a str>,
    pub pageAction: Option<&'a str>,
    pub data: Option<&'a str>,
    pub pageData: Option<&'a str>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Clone, Debug)]
pub enum CloudflareTaskType {
    cf_clearance,
    token,
}
