#![allow(non_snake_case)]

use std::fmt::Debug;
use serde::Deserialize;

use crate::error::GetTaskError::{self, *};
use crate::responses::SvcRespTypeTrait;

#[derive(Deserialize, Clone, Debug)]
pub(crate) struct GetBalanceResp {
    pub(crate) balance: f64,
}

impl SvcRespTypeTrait for GetBalanceResp {
    type Result = f64;
    
    fn get_task_result(&self) -> Self::Result {
        self.balance
    }
}

#[derive(Deserialize, Clone, Debug)]
pub(crate) struct TaskIdResp {
    pub(crate) taskId: u32,
}

impl SvcRespTypeTrait for TaskIdResp {
    type Result = u32;
    
    fn get_task_result(&self) -> Self::Result {
        self.taskId
    }
}

#[derive(Deserialize, Clone, Debug)]
pub(crate) struct GetTaskResultResp<T: TaskTypeTrait> {
    status: TaskState,
    solution: Option<T>,
}

impl<T: TaskTypeTrait> SvcRespTypeTrait for GetTaskResultResp<T> {
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

pub(crate) trait TaskTypeTrait: Clone + Debug {}

impl TaskTypeTrait for ImageToTextTaskResp {}

impl TaskTypeTrait for NoCaptchaTaskProxylessResp {}

impl TaskTypeTrait for NoCaptchaTaskResp {}

impl TaskTypeTrait for RecaptchaV3TaskProxylessResp {}

impl TaskTypeTrait for RecaptchaV2EnterpriseTaskResp {}

impl TaskTypeTrait for RecaptchaV2EnterpriseTaskProxylessResp {}

impl TaskTypeTrait for FunCaptchaTaskResp {}

impl TaskTypeTrait for FunCaptchaTaskProxylessResp {}

impl TaskTypeTrait for HCaptchaTaskResp {}

impl TaskTypeTrait for HCaptchaTaskProxylessResp {}

impl TaskTypeTrait for GeeTestTaskResp {}

impl TaskTypeTrait for GeeTestTaskProxylessResp {}

impl TaskTypeTrait for TurnstileTaskResp {}

impl TaskTypeTrait for TurnstileTaskProxylessResp {}

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

#[derive(Deserialize, Clone, Debug)]
pub struct TurnstileTaskResp {
    pub token: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct TurnstileTaskProxylessResp {
    pub token: String,
}
