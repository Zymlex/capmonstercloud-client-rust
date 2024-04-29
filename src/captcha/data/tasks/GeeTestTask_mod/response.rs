use serde::Deserialize;
use crate::*;

impl TaskRespTrait for GeeTestTaskResp {}
impl TaskRespTrait for GeeTestTaskProxylessResp {}

#[derive(Deserialize, Clone, Debug)]
pub struct GeeTestTaskResp {
    pub challenge: String,
    pub validate: String,
    pub seccode: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GeeTestTaskProxylessResp {
    pub challenge: String,
    pub validate: String,
    pub seccode: String,
}