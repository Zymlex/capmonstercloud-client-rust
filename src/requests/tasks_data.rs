#![allow(non_snake_case)]

use serde::Serialize;
use serde_with_macros::skip_serializing_none;

use crate::modules::Modules;
use crate::proxy_type::ProxyType;
use crate::requests::TaskReqTrait;

#[derive(Serialize)]
pub(crate) enum TaskType<'a> {
    ImageToTextTask(ImageToTextTaskReq<'a>),
    NoCaptchaTaskProxyless(NoCaptchaTaskProxylessReq<'a>),
    NoCaptchaTask(NoCaptchaTaskReq<'a>),
    RecaptchaV3TaskProxyless(RecaptchaV3TaskProxylessReq<'a>),
    RecaptchaV2EnterpriseTask(RecaptchaV2EnterpriseTaskReq<'a>),
    RecaptchaV2EnterpriseTaskProxyless(RecaptchaV2EnterpriseTaskProxylessReq<'a>),
    FunCaptchaTask(FunCaptchaTaskReq<'a>),
    FunCaptchaTaskProxyless(FunCaptchaTaskProxylessReq<'a>),
    HCaptchaTask(HCaptchaTaskReq<'a>),
    HCaptchaTaskProxyless(HCaptchaTaskProxylessReq<'a>),
    GeeTestTask(GeeTestTaskReq<'a>),
    GeeTestTaskProxyless(GeeTestTaskProxylessReq<'a>),
}

impl<'a> TaskReqTrait for ImageToTextTaskReq<'a> {}

impl<'a> TaskReqTrait for NoCaptchaTaskProxylessReq<'a> {}

impl<'a> TaskReqTrait for NoCaptchaTaskReq<'a> {}

impl<'a> TaskReqTrait for RecaptchaV3TaskProxylessReq<'a> {}

impl<'a> TaskReqTrait for RecaptchaV2EnterpriseTaskReq<'a> {}

impl<'a> TaskReqTrait for RecaptchaV2EnterpriseTaskProxylessReq<'a> {}

impl<'a> TaskReqTrait for FunCaptchaTaskReq<'a> {}

impl<'a> TaskReqTrait for FunCaptchaTaskProxylessReq<'a> {}

impl<'a> TaskReqTrait for HCaptchaTaskReq<'a> {}

impl<'a> TaskReqTrait for HCaptchaTaskProxylessReq<'a> {}

impl<'a> TaskReqTrait for GeeTestTaskReq<'a> {}

impl<'a> TaskReqTrait for GeeTestTaskProxylessReq<'a> {}

#[skip_serializing_none]
#[derive(Serialize, Default)]
pub struct ImageToTextTaskReq<'a> {
    pub body: &'a str,
    pub CapMonsterModule: Option<Modules>,
    pub recognizingThreshold: Option<u8>,
    pub Case: Option<bool>,
    pub numeric: Option<u8>,
    pub math: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Default)]
pub struct NoCaptchaTaskProxylessReq<'a> {
    pub websiteURL: &'a str,
    pub websiteKey: &'a str,
    pub recaptchaDataSValue: Option<&'a str>,
    pub userAgent: Option<&'a str>,
    pub cookies: Option<&'a str>,
}

#[skip_serializing_none]
#[derive(Serialize, Default)]
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
#[derive(Serialize, Default)]
pub struct RecaptchaV3TaskProxylessReq<'a> {
    pub websiteURL: &'a str,
    pub websiteKey: &'a str,
    pub minScore: Option<f32>,
    pub pageAction: Option<&'a str>,
}

#[skip_serializing_none]
#[derive(Serialize, Default)]
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
#[derive(Serialize, Default)]
pub struct RecaptchaV2EnterpriseTaskProxylessReq<'a> {
    pub websiteURL: &'a str,
    pub websiteKey: &'a str,
    pub enterprisePayload: Option<&'a str>,
    pub apiDomain: Option<&'a str>,
}

#[skip_serializing_none]
#[derive(Serialize, Default)]
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
#[derive(Serialize, Default)]
pub struct FunCaptchaTaskProxylessReq<'a> {
    pub websiteURL: &'a str,
    pub funcaptchaApiJSSubdomain: Option<&'a str>,
    pub websitePublicKey: &'a str,
    pub data: Option<&'a str>,
}

#[skip_serializing_none]
#[derive(Serialize, Default)]
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
#[derive(Serialize, Default)]
pub struct HCaptchaTaskProxylessReq<'a> {
    pub websiteURL: &'a str,
    pub websiteKey: &'a str,
    pub isInvisible: Option<bool>,
    pub data: Option<&'a str>,
    pub userAgent: Option<&'a str>,
    pub cookies: Option<&'a str>,
}

#[skip_serializing_none]
#[derive(Serialize, Default)]
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
#[derive(Serialize, Default)]
pub struct GeeTestTaskProxylessReq<'a> {
    pub websiteURL: &'a str,
    pub gt: &'a str,
    pub challenge: &'a str,
    pub geetestApiServerSubdomain: &'a str,
    pub geetestGetLib: &'a str,
    pub userAgent: Option<&'a str>,
}
