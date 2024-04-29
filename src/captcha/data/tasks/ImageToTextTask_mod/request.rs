use crate::{ITT_Modules, TaskReqTrait};
use serde::Serialize;
use serde_with_macros::skip_serializing_none;

impl<'a> TaskReqTrait for ImageToTextTask<'a> {}

#[skip_serializing_none]
#[derive(Serialize, Default, Clone, Debug)]
#[serde(tag = "type")]
pub struct ImageToTextTask<'a> {
    pub body: &'a str,
    pub CapMonsterModule: Option<ITT_Modules>,
    pub recognizingThreshold: Option<u8>,
    pub Case: Option<bool>,
    pub numeric: Option<u8>,
    pub math: Option<bool>,
}
