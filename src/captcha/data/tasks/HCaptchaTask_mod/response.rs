use crate::*;
use serde::Deserialize;

impl TaskRespTrait for HCaptchaTaskResp {}

impl TaskRespTrait for HCaptchaTaskProxylessResp {}

#[derive(Deserialize, Clone, Debug)]
pub struct HCaptchaTaskResp {
    pub gRecaptchaResponse: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct HCaptchaTaskProxylessResp {
    pub gRecaptchaResponse: String,
}
