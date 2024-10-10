use crate::*;
use serde::Deserialize;

impl TaskRespTrait for TurnstileTaskResp {}

#[derive(Deserialize, Clone, Debug)]
pub struct TurnstileTaskResp {
    pub token: String,
    pub cf_clearance: String,
}