use crate::*;
use serde::Serialize;
use serde_with_macros::skip_serializing_none;

impl<'a> TaskReqTrait for RecaptchaV2TaskProxyless<'a> {}

impl<'a> TaskReqTrait for RecaptchaV2Task<'a> {}

impl<'a> TaskReqTrait for RecaptchaV2EnterpriseTask<'a> {}

impl<'a> TaskReqTrait for RecaptchaV2EnterpriseTaskProxyless<'a> {}

impl<'a> TaskReqTrait for RecaptchaV3TaskProxyless<'a> {}

#[skip_serializing_none]
#[derive(Serialize, Default, Clone, Debug)]
#[serde(tag = "type")]
pub struct RecaptchaV2TaskProxyless<'a> {
    pub websiteURL: &'a str,
    pub websiteKey: &'a str,
    pub recaptchaDataSValue: Option<&'a str>,
    pub userAgent: Option<&'a str>,
    pub cookies: Option<&'a str>,
    pub isInvisible: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Default, Clone, Debug)]
#[serde(tag = "type")]
pub struct RecaptchaV2Task<'a> {
    pub websiteURL: &'a str,
    pub websiteKey: &'a str,
    pub recaptchaDataSValue: Option<&'a str>,

    #[serde(flatten)]
    pub proxy: ProxySettings<'a>,

    pub userAgent: Option<&'a str>,
    pub cookies: Option<&'a str>,
    pub isInvisible: Option<&'a str>,
}

#[skip_serializing_none]
#[derive(Serialize, Default, Clone, Debug)]
#[serde(tag = "type")]
pub struct RecaptchaV2EnterpriseTask<'a> {
    pub websiteURL: &'a str,
    pub websiteKey: &'a str,
    pub enterprisePayload: Option<&'a str>,
    pub apiDomain: Option<&'a str>,

    #[serde(flatten)]
    pub proxy: ProxySettings<'a>,

    pub userAgent: Option<&'a str>,
    pub cookies: Option<&'a str>,
}

#[skip_serializing_none]
#[derive(Serialize, Default, Clone, Debug)]
#[serde(tag = "type")]
pub struct RecaptchaV2EnterpriseTaskProxyless<'a> {
    pub websiteURL: &'a str,
    pub websiteKey: &'a str,
    pub enterprisePayload: Option<&'a str>,
    pub apiDomain: Option<&'a str>,
}

#[skip_serializing_none]
#[derive(Serialize, Default, Clone, Debug)]
#[serde(tag = "type")]
pub struct RecaptchaV3TaskProxyless<'a> {
    pub websiteURL: &'a str,
    pub websiteKey: &'a str,
    pub minScore: Option<f32>,
    pub pageAction: Option<&'a str>,
}
