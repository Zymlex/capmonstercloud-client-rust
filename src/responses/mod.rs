#![allow(non_snake_case)]

use serde::Deserialize;
use crate::error::GetTaskError;

use crate::error::response_error::ResponseErrorVariants::*;
use crate::error::response_error::*;
use crate::responses::tasks_data::*;

pub(crate) mod tasks_data;

pub(crate) trait RespDataTrait: Clone {
    type Result;
    
    fn get_task_result(&self) -> Self::Result;
}

#[derive(Deserialize, Debug)]
pub(crate) struct Response<T: RespDataTrait> {
    #[cfg(feature = "reserve_response_body")]
    body: String,
    errorId: u32,
    errorCode: Option<String>,
    errorDescription: Option<String>,
    #[serde(flatten)]
    flatten: Option<T>,
}

impl<'a, T: RespDataTrait> Response<T> {
    pub(crate) fn get_result(&'a self) -> Result<T, ResponseError> {
        if self.errorId == 0 {
            if let Some(r) = self.flatten.clone() {
                Ok(r)
            } else {
                Err(ResponseError::new(
                    SuccessResponseWithoutData,
                    #[cfg(feature = "reserve_response_body")]
                    self.body.clone(),
                ))
            }
        } else {
            Err(ResponseError::new(
                Error(ErrorInfo::new(
                    self.errorCode.clone(),
                    self.errorDescription.clone(),
                )),
                #[cfg(feature = "reserve_response_body")]
                self.body.clone(),
            ))
        }
    }
}

pub(crate) trait ResponseTrait {
    // fn get_result<T: RespDataTrait>(&self) -> Result<T, ResponseError>;
}

impl ResponseTrait for Response<GetTaskResultResp<ImageToTextTaskResp>> {
    // fn get_result<T>(&self) -> Result<T, ResponseError> {
    //     self.get_result()?.get_task_result()
    // }
}

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
