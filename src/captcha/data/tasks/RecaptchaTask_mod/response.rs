use crate::*;
use serde::Deserialize;

impl TaskRespTrait for RecaptchaV2TaskProxylessResp {}

// impl TaskRespTrait for RecaptchaV2TaskResp {}

// impl TaskRespTrait for RecaptchaV2EnterpriseTaskResp {}

impl TaskRespTrait for RecaptchaV2EnterpriseTaskProxylessResp {}

impl TaskRespTrait for RecaptchaV3TaskProxylessResp {}

// #[derive(Deserialize, Clone, Debug)]
// pub struct RecaptchaV2TaskResp {
//     pub gRecaptchaResponse: String,
// }

#[derive(Deserialize, Clone, Debug)]
pub struct RecaptchaV2TaskProxylessResp {
    pub gRecaptchaResponse: String,
}

// #[derive(Deserialize, Clone, Debug)]
// pub struct RecaptchaV2EnterpriseTaskResp {
//     pub gRecaptchaResponse: String,
// }

#[derive(Deserialize, Clone, Debug)]
pub struct RecaptchaV2EnterpriseTaskProxylessResp {
    pub gRecaptchaResponse: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct RecaptchaV3TaskProxylessResp {
    pub gRecaptchaResponse: String,
}