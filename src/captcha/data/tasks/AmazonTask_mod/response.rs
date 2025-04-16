use crate::TaskRespTrait;
use serde::Deserialize;

impl TaskRespTrait for AmazonTaskProxylessResp {}

#[derive(Deserialize, Clone, Debug)]
pub struct AmazonTaskProxylessResp {
    pub cookies: AmazonTaskCookies,
    pub userAgent: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AmazonTaskCookies {
    #[serde(rename = "aws-waf-token")]
    pub aws_waf_token: String,
}
