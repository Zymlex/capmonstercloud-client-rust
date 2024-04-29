#![allow(non_snake_case)]

use serde::Deserialize;

use crate::errors::ResponseError;
use crate::responses::tasks_data::*;

pub(crate) mod tasks_data;

pub(crate) trait RespTaskDataTrait: Clone {}

#[derive(Deserialize)]
pub(crate) struct Response<T: RespTaskDataTrait> {
    errorId: u32,
    errorCode: Option<String>,
    errorDescription: Option<String>,
    #[serde(flatten)]
    flatten: Option<T>,
}

pub(crate) trait ResponseTrait {}

impl ResponseTrait for Response<GetTaskResultResp<ImageToTextTaskResp>> {}

impl ResponseTrait for Response<GetTaskResultResp<NoCaptchaTaskProxylessResp>> {}

impl ResponseTrait for Response<GetTaskResultResp<NoCaptchaTaskResp>> {}

impl ResponseTrait for Response<GetTaskResultResp<RecaptchaV3TaskProxylessResp>> {}

impl ResponseTrait for Response<GetTaskResultResp<RecaptchaV2EnterpriseTaskResp>> {}

impl ResponseTrait for Response<GetTaskResultResp<RecaptchaV2EnterpriseTaskProxylessResp>> {}

impl ResponseTrait for Response<GetTaskResultResp<FunCaptchaTaskResp>> {}

impl ResponseTrait for Response<GetTaskResultResp<FunCaptchaTaskProxylessResp>> {}

impl ResponseTrait for Response<GetTaskResultResp<HCaptchaTaskResp>> {}

impl ResponseTrait for Response<GetTaskResultResp<HCaptchaTaskProxylessResp>> {}

impl ResponseTrait for Response<GetTaskResultResp<GeeTestTaskResp>> {}

impl ResponseTrait for Response<GetTaskResultResp<GeeTestTaskProxylessResp>> {}


impl<'a, T: RespTaskDataTrait> Response<T> {
    pub(crate) fn get_result(&'a self) -> Result<T, ResponseError> {
        if self.errorId == 0 {
            if let Some(r) = self.flatten.clone() {
                Ok(r)
            } else {
                panic!()
            }
        } else {
            panic!()
        }
    }
}