#![allow(non_snake_case)]

mod parse;

use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::{fmt::{Debug, Display, Formatter}, num::NonZeroU8};

use crate::error::{SvcResponseError, SvcRespStructError};

#[derive(Debug, Clone)]
pub struct SvcResponse<X: SvcRespTypeTrait + DeserializeOwned> {
    resp: SvcResp<X>,
}

impl<X: SvcRespTypeTrait + DeserializeOwned> SvcResponse<X> {
    pub const fn new(
        svc_struct: SvcResp<X>,
    ) -> Self {
        Self {
            resp: svc_struct,
        }
    }

    pub fn get_result(&self) -> Result<X::Value, SvcResponseError>
    {
        match &self.resp {
            SvcResp::Success(r) => r.get_result().map_err(|e| {
                SvcResponseError::GettingResultError(
                    e,
                )
            }),
            SvcResp::Error(e) => Err(SvcResponseError::SvcReturnErrorCode(e.clone())),
        }
    }

    pub const fn get_inner(&self) -> &SvcResp<X> {
        &self.resp
    }
}

pub trait SvcRespTypeTrait: Clone + Debug {
    // Required for explicit use `Result` type to support `Try`.
    type Value;

    fn get_result(&self) -> Result<Self::Value, SvcRespStructError>;
}

#[derive(Debug, Clone)]
pub enum SvcResp<X: SvcRespTypeTrait> {
    Success(SvcSuccessResp<X>),
    Error(SvcErrorResp),
}

// Success

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
{
    pub(crate) fn get_result(&self) -> Result<X::Value, SvcRespStructError> {
        match &self.flat_data {
            Some(r) => Ok(r.get_result()?),
            None => Err(SvcRespStructError::_ReportBugSuccessResponseWithoutData),
        }
    }
}

// Error

#[derive(Deserialize, Debug, Clone)]
pub struct SvcErrorResp {
    #[serde(deserialize_with = "parse::check_failure_errorId")]
    errorId: NonZeroU8,
    errorCode: Option<String>,
    errorDescription: Option<String>,
    taskId: Option<u64>,
}

impl SvcErrorResp {
    const fn get_error_id(&self) -> u8 {
        self.errorId.get()
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
