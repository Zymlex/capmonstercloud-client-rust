#![allow(non_snake_case)]

mod parse;

pub(crate) use crate::captcha::request::get_task_result_repeater::*;

use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::fmt::{Debug, Display, Formatter};

use crate::error::{SvcResponseError, SvcRespStructError};

#[derive(Debug, Clone)]
pub struct SvcResponse<X: SvcRespTypeTrait + DeserializeOwned> {
    resp: SvcResp<X>,
    #[cfg(feature = "keep-request-body")]
    body: String,
}

impl<X: SvcRespTypeTrait + DeserializeOwned> SvcResponse<X> {
    pub const fn new(
        svc_struct: SvcResp<X>,
        #[cfg(feature = "keep-request-body")] body: String,
    ) -> Self {
        Self {
            resp: svc_struct,
            #[cfg(feature = "keep-request-body")]
            body,
        }
    }

    pub fn get_result(&self) -> Result<X::Value, SvcResponseError>
    where
        SvcRespStructError: From<<X as SvcRespTypeTrait>::Error>,
    {
        match &self.resp {
            SvcResp::Success(r) => r.get_result().map_err(|e| {
                SvcResponseError::GettingResultError(
                    e,
                    #[cfg(feature = "keep-request-body")]
                    self.body.clone(),
                )
            }),
            SvcResp::Error(e) => Err(SvcResponseError::SvcReturnErrorCode(e.clone())),
        }
    }

    pub const fn get_struct(&self) -> &SvcResp<X> {
        &self.resp
    }

    pub fn get_body(&self) -> &str {
        #[cfg(feature = "keep-request-body")]
        return &self.body;

        #[cfg(not(feature = "keep-request-body"))]
        compile_error!("Required 'keep-request-body' feature.")
    }
}

// impl<'a, T: TaskTypeTrait + DeserializeOwned + 'a> SvcResponse<GetTaskResultResp<'a, T>> {
//     fn check_task(&self) -> Result<(),SvcResponseError> {
//         match self.get_result() {
//             Ok(r) => match r {
//                 Ok(r) => return Ok(r),
//                 Err(e) => match e {
//                     GetTaskError::Processing => continue,
//                     GetTaskError::ReadyTaskWithoutSolution => {
//                         return Err(TaskResultError::ReadyTaskWithoutSolution)
//                     }
//                 },
//             },
//             Err(e) => return Err(TaskResultError::GetResultFailed(e)),
//         }
//     }
// }

pub trait SvcRespTypeTrait: Clone + Debug {
    // Required for explicit use `Result` type to support `Try`. type Value; type Error;
    type Value;
    type Error;

    fn get_task_result(&self) -> Result<Self::Value, Self::Error>;
}

// pub(crate) trait TaskRespTypeTrait: SvcRespTypeTrait {
//     type TaskRespType: TaskRespTrait + DeserializeOwned;
// }

#[derive(Debug, Clone)]
pub enum SvcResp<X: SvcRespTypeTrait> {
    Success(SvcSuccessResp<X>),
    Error(SvcErrorResp),
}

#[derive(Deserialize, Debug, Clone)]
pub struct SvcSuccessResp<X: SvcRespTypeTrait> {
    /// Always 0 and required only for easy check.
    #[serde(deserialize_with = "parse::check_success_errorId")]
    #[allow(dead_code)]
    #[doc(hidden)]
    errorId: (),
    #[serde(flatten)]
    flat_data: Option<X>,
}

impl<X: SvcRespTypeTrait + DeserializeOwned> SvcSuccessResp<X>
where
    SvcRespStructError: From<<X as SvcRespTypeTrait>::Error>,
{
    pub(crate) fn get_result(&self) -> Result<X::Value, SvcRespStructError> {
        match &self.flat_data {
            Some(r) => Ok(r.get_task_result()?),
            None => Err(SvcRespStructError::SuccessResponseWithoutData),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct SvcErrorResp {
    #[serde(deserialize_with = "parse::check_failure_errorId")]
    errorId: u8,
    errorCode: Option<String>,
    errorDescription: Option<String>,
    taskId: Option<u64>,
}

impl SvcErrorResp {
    const fn get_error_id(&self) -> u8 {
        self.errorId
    }

    fn get_error_code(&self) -> &str {
        self.errorCode.as_deref().unwrap_or("n/a")
    }

    fn get_error_description(&self) -> Option<&str> {
        self.errorDescription.as_deref()
    }

    const fn get_task_id(&self) -> Option<u64> {
        self.taskId
    }
}

impl Display for SvcErrorResp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut text = format!(
            "errorId: {}\rerrorCode: {}",
            self.get_error_id(),
            self.get_error_code()
        );

        if let Some(r) = self.get_error_description() {
            text.push_str(r);
        }

        if let Some(r) = self.get_task_id() {
            text.push_str(&r.to_string());
        }

        write!(f, "{text}")
    }
}
