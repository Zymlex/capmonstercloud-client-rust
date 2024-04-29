use crate::*;
use serde::Serialize;
use serde_with_macros::skip_serializing_none;

impl<'a> TaskReqTrait for Turnstile<'a> {}
impl<'a> TaskReqTrait for CloudFlareChallenge<'a> {}
impl<'a> TaskReqTrait for CloudFlareChallengeWithProxy<'a> {}

#[skip_serializing_none]
#[derive(Serialize, Default, Clone, Debug)]
#[serde(tag = "type")]
pub struct Turnstile<'a> {
    pub websiteURL: &'a str,
    pub websiteKey: &'a str,
    pub pageAction: Option<&'a str>,
}

#[skip_serializing_none]
#[derive(Serialize, Default, Clone, Debug)]
#[serde(tag = "type")]
pub struct CloudFlareChallenge<'a> {
    pub websiteURL: &'a str,
    pub websiteKey: &'a str,
    pub cloudflareTaskType: &'a str,
    pub userAgent: &'a str,
    pub pageAction: &'a str,
    pub data: &'a str,
    pub pageData: &'a str,
}

#[skip_serializing_none]
#[derive(Serialize, Default, Clone, Debug)]
#[serde(tag = "type")]
pub struct CloudFlareChallengeWithProxy<'a> {
    pub websiteURL: &'a str,
    pub websiteKey: &'a str,
    
    pub cloudflareTaskType: Option<CloudflareTaskType>,

    #[serde(flatten)]
    pub proxy: ProxySettings<'a>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Clone, Debug)]
pub enum CloudflareTaskType {
    cf_clearance,
    token,
}
