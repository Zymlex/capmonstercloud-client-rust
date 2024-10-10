use std::fmt::Debug;

use crate::*;
use serde::Serialize;
use serde_with_macros::skip_serializing_none;

impl<'a> TaskReqTrait for TurnstileTask<'a> {}

#[skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub(crate)  struct TurnstileTask<'a> {
    pub r#type: &'a str,

    #[serde(flatten)]
    pub data: TTData<'a>,

    pub cloudflareTaskType: Option<&'a str>,
}

#[skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub(crate) enum TTData<'a> {
    Turnstile(Turnstile<'a>),
    CloudFlareToken(CloudFlareToken<'a>),
    CloudFlareCFClearance(CloudFlareCFClearance<'a>),
}

#[skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct Turnstile<'a> {
    pub websiteURL: &'a str,
    pub websiteKey: &'a str,
    pub pageAction: Option<&'a str>,
}

/// https://docs.capmonster.cloud/docs/captchas/tunrstile-task#option-2-cloudflare
#[skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct CloudFlareToken<'a> {
    pub websiteURL: &'a str,
    pub websiteKey: &'a str,
    pub userAgent: &'a str,
    pub pageAction: &'a str,
    pub pageData: &'a str,
    pub data: &'a str,
}

/// https://docs.capmonster.cloud/docs/captchas/tunrstile-task#option-3-cloudflare
#[skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
#[serde(tag = "type")]
pub struct CloudFlareCFClearance<'a> {
    pub websiteURL: &'a str,
    pub websiteKey: &'a str,
    pub htmlPageBase64: &'a str,
    pub userAgent: &'a str,

    #[serde(flatten)]
    pub proxy: ProxySettings<'a>,
}
