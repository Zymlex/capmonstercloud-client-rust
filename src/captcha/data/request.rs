#![allow(non_snake_case, reason = "API")]

use crate::*;
use serde::Serialize;

pub(crate) trait MethodReqTrait {}

impl<'a, T: TaskReqTrait> MethodReqTrait for CreateTask<'a, T> {}
impl<'a> MethodReqTrait for GetBalance<'a> {}
impl<'a> MethodReqTrait for GetTaskResult<'a> {}

#[derive(Serialize)]
pub(crate) struct CreateTask<'a, T: TaskReqTrait> {
    pub(crate) softId: &'a str,
    #[allow(non_snake_case, reason = "API")]
    pub(crate) clientKey: &'a str,
    pub(crate) task: T,
    // pub(crate) callbackUrl: Option<Url>,
}

#[derive(Serialize)]
pub(crate) struct GetBalance<'a> {
    #[allow(non_snake_case, reason = "API")]
    pub(crate) clientKey: &'a str,
}

#[derive(Serialize)]
pub(crate) struct GetTaskResult<'a> {
    #[allow(non_snake_case, reason = "API")]
    pub(crate) clientKey: &'a str,
    #[allow(non_snake_case, reason = "API")]
    pub(crate) taskId: u32,
}
