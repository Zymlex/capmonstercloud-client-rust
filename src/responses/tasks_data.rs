#![allow(non_snake_case)]

use serde::Deserialize;

use crate::errors::GetTaskError;
use crate::responses::RespTaskDataTrait;

// #[derive(Deserialize)]
// pub(crate) enum RespAddData {
//     GetBalanceResponse(GetBalanceResp),
//     GetTaskResultResponse(GetTaskResultResp),
// }

impl RespTaskDataTrait for GetBalanceResp {}

impl RespTaskDataTrait for TaskIdResp {}

impl<T: TaskRespTrait> RespTaskDataTrait for GetTaskResultResp<T> {}

#[derive(Deserialize, Clone)]
pub(crate) struct GetBalanceResp {
    pub(crate) balance: f64,
}

#[derive(Deserialize, Clone)]
pub(crate) enum TaskState {
    Processing,
    Ready,
}

#[derive(Deserialize, Clone)]
pub(crate) struct TaskIdResp {
    pub(crate) taskId: u32,
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

#[derive(Deserialize, Clone)]
pub(crate) struct GetTaskResultResp<T: TaskRespTrait> {
    status: TaskState,
    #[serde(flatten)]
    flatten: Option<T>,
}

impl<T: TaskRespTrait> GetTaskResultResp<T> {
    pub(crate) fn get_task_result(&self) -> Result<T, GetTaskError> {
        if let TaskState::Ready = self.status {
            if let Some(r) = self.flatten.clone() {
                Ok(r)
            } else {
                panic!()
            }
        } else if let TaskState::Processing = self.status {
            panic!()
        } else {
            panic!()
        }
    }
}

#[derive(Deserialize, Clone)]
pub struct ImageToTextTaskResp {
    pub text: String,
}

#[derive(Deserialize, Clone)]
pub struct NoCaptchaTaskProxylessResp {
    pub gRecaptchaResponse: String,
}

#[derive(Deserialize, Clone)]
pub struct NoCaptchaTaskResp {
    pub gRecaptchaResponse: String,
}

#[derive(Deserialize, Clone)]
pub struct RecaptchaV3TaskProxylessResp {
    pub gRecaptchaResponse: String,
}

#[derive(Deserialize, Clone)]
pub struct RecaptchaV2EnterpriseTaskResp {
    pub gRecaptchaResponse: String,
}

#[derive(Deserialize, Clone)]
pub struct RecaptchaV2EnterpriseTaskProxylessResp {
    pub gRecaptchaResponse: String,
}

#[derive(Deserialize, Clone)]
pub struct FunCaptchaTaskResp {
    pub token: String,
}

#[derive(Deserialize, Clone)]
pub struct FunCaptchaTaskProxylessResp {
    pub token: String,
}

#[derive(Deserialize, Clone)]
pub struct HCaptchaTaskResp {
    pub gRecaptchaResponse: String,
}

#[derive(Deserialize, Clone)]
pub struct HCaptchaTaskProxylessResp {
    pub gRecaptchaResponse: String,
}

#[derive(Deserialize, Clone)]
pub struct GeeTestTaskResp {
    pub challenge: String,
    pub validate: String,
    pub seccode: String,
}

#[derive(Deserialize, Clone)]
pub struct GeeTestTaskProxylessResp {
    pub challenge: String,
    pub validate: String,
    pub seccode: String,
}