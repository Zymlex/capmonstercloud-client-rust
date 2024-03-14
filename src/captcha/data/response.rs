#![allow(non_snake_case)]

use crate::error::GetTaskError;
use crate::{SvcRespTypeTrait, TaskRespTrait};
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct GetBalanceResp {
    pub(crate) balance: f64,
}

#[derive(Deserialize, Clone, Debug)]
pub(crate) struct TaskIdResp {
    pub(crate) taskId: u32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GetTaskResultResp<Y: TaskRespTrait> {
    status: TaskState,
    solution: Option<Y>,
}

impl SvcRespTypeTrait for GetBalanceResp {
    type Value = f64;
    type Error = ();

    fn get_task_result(&self) -> Result<Self::Value, Self::Error> {
        Ok(self.balance)
    }
}

impl<'a> SvcRespTypeTrait for TaskIdResp {
    type Value = u32;
    type Error = ();

    fn get_task_result(&self) -> Result<Self::Value, Self::Error> {
        Ok(self.taskId)
    }
}

impl<'a, Y: TaskRespTrait + 'a> SvcRespTypeTrait for GetTaskResultResp<Y> {
    type Value = Y;
    type Error = GetTaskError;

    fn get_task_result(&self) -> Result<Self::Value, Self::Error> {
        let status = &self.status;

        match status {
            TaskState::ready =>
            {
                #[allow(clippy::option_if_let_else)]
                if let Some(r) = &self.solution {
                    Ok(r.clone())
                } else {
                    Err(GetTaskError::ReadyTaskWithoutSolution)
                }
            }
            TaskState::processing => Err(GetTaskError::Processing),
        }
    }
}

// impl<Y: TaskRespTrait + DeserializeOwned> TaskRespTypeTrait for GetTaskResultResp<Y> {
//     type TaskRespType = Y;
// }

#[allow(non_camel_case_types)]
#[derive(Deserialize, Clone, Debug)]
pub(crate) enum TaskState {
    processing,
    ready,
}