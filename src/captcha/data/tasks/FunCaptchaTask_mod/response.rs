use crate::TaskRespTrait;
use serde::Deserialize;

impl TaskRespTrait for FunCaptchaTaskResp {}

impl TaskRespTrait for FunCaptchaTaskProxylessResp {}

#[derive(Deserialize, Clone, Debug)]
pub struct FunCaptchaTaskResp {
    pub token: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct FunCaptchaTaskProxylessResp {
    pub token: String,
}
