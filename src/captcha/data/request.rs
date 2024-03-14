#![allow(non_snake_case)]

use serde::Serialize;
use crate::TaskReqTrait;

pub(crate) trait MethodReqTrait {}

impl<'a, T: TaskReqTrait> MethodReqTrait for CreateTask<'a, T> {}
impl<'a> MethodReqTrait for GetBalance<'a> {}
impl<'a> MethodReqTrait for GetTaskResult<'a> {}

#[derive(Serialize)]
pub(crate) struct CreateTask<'a, T: TaskReqTrait> {
    pub(crate) softId: &'a str,
    pub(crate) clientKey: &'a str,
    pub(crate) task: T,
 // pub(crate) callbackUrl: Option<Url>,
}

#[derive(Serialize)]
pub(crate) struct GetBalance<'a> {
    pub(crate) clientKey: &'a str,
}

#[derive(Serialize)]
pub(crate) struct GetTaskResult<'a> {
    pub(crate) clientKey: &'a str,
    pub(crate) taskId: u32,
}