use crate::*;
use serde::Deserialize;

impl TaskRespTrait for ImageToTextTaskResp {}

#[derive(Deserialize, Clone, Debug)]
pub struct ImageToTextTaskResp {
    pub text: String,
}
