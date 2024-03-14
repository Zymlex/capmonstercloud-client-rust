use crate::{ProxySettings, TaskReqTrait};
use serde::Serialize;
use serde_with_macros::skip_serializing_none;

impl<'a> TaskReqTrait for GeeTestTask<'a> {}

impl<'a> TaskReqTrait for GeeTestTaskProxyless<'a> {}

#[skip_serializing_none]
#[derive(Serialize, Default, Clone, Debug)]
#[serde(tag = "type")]
pub struct GeeTestTask<'a> {
    pub websiteURL: &'a str,
    pub gt: &'a str,
    pub challenge: Option<&'a str>,
    pub geetestApiServerSubdomain: &'a str,
    pub geetestGetLib: &'a str,
    pub version: Option<u32>,
    pub initParameters: Option<GeeTestTaskInitParameters<'a>>,

    #[serde(flatten)]
    pub proxy: ProxySettings<'a>,

    pub userAgent: Option<&'a str>,
}

#[skip_serializing_none]
#[derive(Serialize, Default, Clone, Debug)]
#[serde(tag = "type")]
pub struct GeeTestTaskProxyless<'a> {
    pub websiteURL: &'a str,
    pub gt: &'a str,
    pub challenge: &'a str,
    pub geetestApiServerSubdomain: Option<&'a str>,
    pub geetestGetLib: Option<&'a str>,
    pub version: Option<u32>,
    pub initParameters: Option<GeeTestTaskInitParameters<'a>>,
    pub userAgent: Option<&'a str>,
}

#[skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct GeeTestTaskInitParameters<'a> {
    pub riskType: &'a str,
}
