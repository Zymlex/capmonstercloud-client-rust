#![allow(non_snake_case)]

use serde::Deserialize;

use crate::error::GetTaskError::{self, *};
use crate::responses::RespDataTrait;

#[derive(Deserialize, Clone, Debug)]
pub(crate) struct GetBalanceResp {
    pub(crate) balance: f64,
}

impl RespDataTrait for GetBalanceResp {
    type Result = Result<f64, ()>;
    
    fn get_task_result(&self) -> Self::Result {
        Ok(self.balance)
    }
}

#[derive(Deserialize, Clone, Debug)]
pub(crate) struct TaskIdResp {
    pub(crate) taskId: u32,
}

impl RespDataTrait for TaskIdResp {
    type Result = Result<u32, ()>;
    
    fn get_task_result(&self) -> Self::Result {
        Ok(self.taskId)
    }
}

#[derive(Deserialize, Clone, Debug)]
pub(crate) struct GetTaskResultResp<T: TaskRespTrait> {
    status: TaskState,
    solution: Option<T>,
}

impl<T: TaskRespTrait> RespDataTrait for GetTaskResultResp<T> {
    type Result = Result<T, GetTaskError>;
    
    fn get_task_result(&self) -> Self::Result {
        match self.status {
            TaskState::Ready => {
                if let Some(r) = self.solution.clone() {
                    Ok(r)
                } else {
                    Err(ReadyTaskWithoutSolution)
                }
            }
            TaskState::Processing => {
                Err(Processing)
            }
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
pub(crate) enum TaskState {
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "ready")]
    Ready,
}

pub(crate) trait TaskRespTrait: Clone {}

impl TaskRespTrait for ImageToTextTaskResp {}

impl TaskRespTrait for NoCaptchaTaskProxylessResp {}

impl TaskRespTrait for NoCaptchaTaskResp {}

impl TaskRespTrait for RecaptchaV3TaskProxylessResp {}

impl TaskRespTrait for RecaptchaV2EnterpriseTaskResp {}

impl TaskRespTrait for RecaptchaV2EnterpriseTaskProxylessResp {}

impl TaskRespTrait for FunCaptchaTaskResp {}

impl TaskRespTrait for FunCaptchaTaskProxylessResp {}

impl TaskRespTrait for HCaptchaTaskResp {}

impl TaskRespTrait for HCaptchaTaskProxylessResp {}

impl TaskRespTrait for GeeTestTaskResp {}

impl TaskRespTrait for GeeTestTaskProxylessResp {}

#[derive(Deserialize, Clone, Debug)]
pub struct ImageToTextTaskResp {
    pub text: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct NoCaptchaTaskProxylessResp {
    pub gRecaptchaResponse: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct NoCaptchaTaskResp {
    pub gRecaptchaResponse: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct RecaptchaV3TaskProxylessResp {
    pub gRecaptchaResponse: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct RecaptchaV2EnterpriseTaskResp {
    pub gRecaptchaResponse: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct RecaptchaV2EnterpriseTaskProxylessResp {
    pub gRecaptchaResponse: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct FunCaptchaTaskResp {
    pub token: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct FunCaptchaTaskProxylessResp {
    pub token: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct HCaptchaTaskResp {
    pub gRecaptchaResponse: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct HCaptchaTaskProxylessResp {
    pub gRecaptchaResponse: String,
}

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
