#![allow(non_snake_case, reason = "API")]

use crate::error::GetTaskError;
use crate::*;
use error::SvcRespStructError;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct GetBalanceResp {
    pub(crate) balance: f64,
}

impl SvcRespTypeTrait for GetBalanceResp {
    type Value = f64;

    fn get_result(&self) -> Result<Self::Value, SvcRespStructError> {
        Ok(self.balance)
    }
}

#[derive(Deserialize, Clone, Debug)]
pub(crate) struct TaskIdResp {
    pub(crate) taskId: u32,
}

impl SvcRespTypeTrait for TaskIdResp {
    type Value = u32;

    fn get_result(&self) -> Result<Self::Value, SvcRespStructError> {
        Ok(self.taskId)
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct GetTaskResultResp<Y: TaskRespTrait> {
    status: TaskState,
    solution: Option<Y>,
}

impl<'a, Y: TaskRespTrait + 'a> SvcRespTypeTrait for GetTaskResultResp<Y> {
    type Value = Y;

    fn get_result(&self) -> Result<Self::Value, SvcRespStructError> {
        let status = &self.status;

        match status {
            TaskState::ready =>
            {
                #[allow(clippy::option_if_let_else, reason = "Readability")]
                if let Some(r) = &self.solution {
                    Ok(r.clone())
                } else {
                    Err(SvcRespStructError::GetTaskError(
                        GetTaskError::ReadyTaskWithoutSolution,
                    ))
                }
            }
            TaskState::processing => {
                Err(SvcRespStructError::GetTaskError(GetTaskError::Processing))
            }
        }
    }
}

#[allow(non_camel_case_types, reason = "API")]
#[derive(Deserialize, Clone, Debug)]
pub(crate) enum TaskState {
    processing,
    ready,
}
