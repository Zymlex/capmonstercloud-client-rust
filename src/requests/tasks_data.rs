#![allow(non_snake_case)]

use serde::Serialize;
use serde_with_macros::skip_serializing_none;
use std::fmt::Debug;

use crate::modules::Modules;
use crate::proxy_type::ProxyType;

// #[derive(Serialize)]
// pub(crate) enum TaskType<'a> {
//     ImageToTextTask(ImageToTextTaskReq<'a>),
//     NoCaptchaTaskProxyless(NoCaptchaTaskProxylessReq<'a>),
//     NoCaptchaTask(NoCaptchaTaskReq<'a>),
//     RecaptchaV3TaskProxyless(RecaptchaV3TaskProxylessReq<'a>),
//     RecaptchaV2EnterpriseTask(RecaptchaV2EnterpriseTaskReq<'a>),
//     RecaptchaV2EnterpriseTaskProxyless(RecaptchaV2EnterpriseTaskProxylessReq<'a>),
//     FunCaptchaTask(FunCaptchaTaskReq<'a>),
//     FunCaptchaTaskProxyless(FunCaptchaTaskProxylessReq<'a>),
//     HCaptchaTask(HCaptchaTaskReq<'a>),
//     HCaptchaTaskProxyless(HCaptchaTaskProxylessReq<'a>),
//     GeeTestTask(GeeTestTaskReq<'a>),
//     GeeTestTaskProxyless(GeeTestTaskProxylessReq<'a>),
// }

#[skip_serializing_none]
#[derive(Serialize, Default, Clone, Debug)]
pub struct ImageToTextTaskReq<'a> {
    pub body: &'a str,
    pub CapMonsterModule: Option<Modules>,
    pub recognizingThreshold: Option<u8>,
    pub Case: Option<bool>,
    pub numeric: Option<u8>,
    pub math: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Default, Clone, Debug)]
pub struct NoCaptchaTaskProxylessReq<'a> {
    pub websiteURL: &'a str,
    pub websiteKey: &'a str,
    pub recaptchaDataSValue: Option<&'a str>,
    pub userAgent: Option<&'a str>,
    pub cookies: Option<&'a str>,
}

#[skip_serializing_none]
#[derive(Serialize, Default, Clone, Debug)]
pub struct NoCaptchaTaskReq<'a> {
    pub websiteURL: &'a str,
    pub websiteKey: &'a str,
    pub recaptchaDataSValue: Option<&'a str>,
    pub proxyType: ProxyType,
    pub proxyAddress: &'a str,
    pub proxyPort: u16,
    pub proxyLogin: Option<&'a str>,
    pub proxyPassword: Option<&'a str>,
    pub userAgent: Option<&'a str>,
    pub cookies: Option<&'a str>,
}

#[skip_serializing_none]
#[derive(Serialize, Default, Clone, Debug)]
pub struct RecaptchaV3TaskProxylessReq<'a> {
    pub websiteURL: &'a str,
    pub websiteKey: &'a str,
    pub minScore: Option<f32>,
    pub pageAction: Option<&'a str>,
}

#[skip_serializing_none]
#[derive(Serialize, Default, Clone, Debug)]
pub struct RecaptchaV2EnterpriseTaskReq<'a> {
    pub websiteURL: &'a str,
    pub websiteKey: &'a str,
    pub enterprisePayload: Option<&'a str>,
    pub apiDomain: Option<&'a str>,
    pub proxyType: ProxyType,
    pub proxyAddress: &'a str,
    pub proxyPort: u16,
    pub proxyLogin: Option<&'a str>,
    pub proxyPassword: Option<&'a str>,
    pub userAgent: Option<&'a str>,
    pub cookies: Option<&'a str>,
}

#[skip_serializing_none]
#[derive(Serialize, Default, Clone, Debug)]
pub struct RecaptchaV2EnterpriseTaskProxylessReq<'a> {
    pub websiteURL: &'a str,
    pub websiteKey: &'a str,
    pub enterprisePayload: Option<&'a str>,
    pub apiDomain: Option<&'a str>,
}

#[skip_serializing_none]
#[derive(Serialize, Default, Clone, Debug)]
pub struct FunCaptchaTaskReq<'a> {
    pub websiteURL: &'a str,
    pub funcaptchaApiJSSubdomain: Option<&'a str>,
    pub websitePublicKey: &'a str,
    pub data: Option<&'a str>,
    pub proxyType: ProxyType,
    pub proxyAddress: &'a str,
    pub proxyPort: u16,
    pub proxyLogin: Option<&'a str>,
    pub proxyPassword: Option<&'a str>,
    pub userAgent: Option<&'a str>,
    pub cookies: Option<&'a str>,
}

#[skip_serializing_none]
#[derive(Serialize, Default, Clone, Debug)]
pub struct FunCaptchaTaskProxylessReq<'a> {
    pub websiteURL: &'a str,
    pub funcaptchaApiJSSubdomain: Option<&'a str>,
    pub websitePublicKey: &'a str,
    pub data: Option<&'a str>,
}

#[skip_serializing_none]
#[derive(Serialize, Default, Clone, Debug)]
pub struct HCaptchaTaskReq<'a> {
    pub websiteURL: &'a str,
    pub websiteKey: &'a str,
    pub isInvisible: Option<bool>,
    pub data: Option<&'a str>,
    pub proxyType: ProxyType,
    pub proxyAddress: &'a str,
    pub proxyPort: u16,
    pub proxyLogin: Option<&'a str>,
    pub proxyPassword: Option<&'a str>,
    pub userAgent: Option<&'a str>,
    pub cookies: Option<&'a str>,
}

#[skip_serializing_none]
#[derive(Serialize, Default, Clone, Debug)]
pub struct HCaptchaTaskProxylessReq<'a> {
    pub websiteURL: &'a str,
    pub websiteKey: &'a str,
    pub isInvisible: Option<bool>,
    pub data: Option<&'a str>,
    pub userAgent: Option<&'a str>,
    pub cookies: Option<&'a str>,
}

#[skip_serializing_none]
#[derive(Serialize, Default, Clone, Debug)]
pub struct GeeTestTaskReq<'a> {
    pub websiteURL: &'a str,
    pub gt: &'a str,
    pub challenge: &'a str,
    pub geetestApiServerSubdomain: &'a str,
    pub geetestGetLib: &'a str,
    pub proxyType: ProxyType,
    pub proxyAddress: &'a str,
    pub proxyPort: u16,
    pub proxyLogin: Option<&'a str>,
    pub proxyPassword: Option<&'a str>,
    pub userAgent: Option<&'a str>,
    pub cookies: Option<&'a str>,
}

#[skip_serializing_none]
#[derive(Serialize, Default, Clone, Debug)]
pub struct GeeTestTaskProxylessReq<'a> {
    pub websiteURL: &'a str,
    pub gt: &'a str,
    pub challenge: &'a str,
    pub geetestApiServerSubdomain: &'a str,
    pub geetestGetLib: &'a str,
    pub userAgent: Option<&'a str>,
}

#[skip_serializing_none]
#[derive(Serialize, Default, Clone, Debug)]
pub struct TurnstileTaskReq<'a> {
    pub websiteURL: &'a str,
    pub websiteKey: &'a str,
    pub proxyType: ProxyType,
    pub proxyAddress: &'a str,
    pub proxyPort: u16,
    pub proxyLogin: Option<&'a str>,
    pub proxyPassword: Option<&'a str>,
}

#[skip_serializing_none]
#[derive(Serialize, Default, Clone, Debug)]
pub struct TurnstileTaskProxylessReq<'a> {
    pub websiteURL: &'a str,
    pub websiteKey: &'a str,
}