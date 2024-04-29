use crate::TaskRespTrait;
use serde::Deserialize;

impl TaskRespTrait for NoCaptchaTaskProxylessResp {}

impl TaskRespTrait for NoCaptchaTaskResp {}

impl TaskRespTrait for RecaptchaV2EnterpriseTaskResp {}

impl TaskRespTrait for RecaptchaV2EnterpriseTaskProxylessResp {}

impl TaskRespTrait for RecaptchaV3TaskProxylessResp {}

#[derive(Deserialize, Clone, Debug)]
pub struct NoCaptchaTaskProxylessResp {
    pub gRecaptchaResponse: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct NoCaptchaTaskResp {
    pub gRecaptchaResponse: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct RecaptchaV3TaskProxylessResp {
    pub gRecaptchaResponse: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct RecaptchaV2EnterpriseTaskResp {
    pub gRecaptchaResponse: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct RecaptchaV2EnterpriseTaskProxylessResp {
    pub gRecaptchaResponse: String,
}
