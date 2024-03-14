use crate::TaskRespTrait;
use serde::Deserialize;

impl TaskRespTrait for TurnstileTaskResp {}

impl TaskRespTrait for TurnstileTaskProxylessResp {}

#[derive(Deserialize, Clone, Debug)]
pub struct TurnstileTaskResp {
    pub token: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct TurnstileTaskProxylessResp {
    pub token: String,
}
