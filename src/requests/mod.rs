#![allow(non_snake_case)]

use serde::Serialize;

pub(crate) mod tasks_data;

#[derive(Serialize)]
pub(crate) struct CreateTaskRequest<'a, T: TaskReqTrait> {
    pub(crate) clientKey: &'a str,
    pub(crate) task: TaskData<'a, T>,
    // pub(crate) callbackUrl: Option<&'a str>,
}

pub(crate) trait TaskReqTrait: Serialize {}

#[derive(Serialize)]
pub(crate) struct TaskData<'a, T: TaskReqTrait> {
    #[serde(rename = "type")]
    typ: &'a str,
    #[serde(flatten)]
    flatten: T,
}

impl<'a, T: TaskReqTrait> TaskData<'a, T> {
    pub(crate) fn new(typ: &'a str, flatten: T) -> Self {
        Self { typ, flatten }
    }
}

#[derive(Serialize)]
pub(crate) struct GetBalanceRequest<'a> {
    pub(crate) clientKey: &'a str
}


#[derive(Serialize)]
pub(crate) struct GetTaskResultRequest<'a> {
    pub(crate) clientKey: &'a str,
    pub(crate) taskId: u32,
}