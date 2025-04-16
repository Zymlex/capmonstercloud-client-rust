use crate::TaskReqTrait;
use serde::Serialize;
use serde_with_macros::skip_serializing_none;

impl<'a> TaskReqTrait for AmazonTaskProxyless<'a> {}

#[skip_serializing_none]
#[derive(Serialize, Default, Clone, Debug)]
#[serde(tag = "type")]
pub struct AmazonTaskProxyless<'a> {
    pub websiteURL: &'a str,
    pub challengeScript: &'a str,
    pub captchaScript: &'a str,
    pub websiteKey: &'a str,
    pub context: &'a str,
    pub iv: &'a str,
    pub cookieSolution: Option<bool>,
}
