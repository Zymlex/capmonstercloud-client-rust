use serde::Deserialize;
use crate::TaskRespTrait;

impl TaskRespTrait for ImageToTextTaskResp {}

#[derive(Deserialize, Clone, Debug)]
pub struct ImageToTextTaskResp {
    pub text: String,
}
