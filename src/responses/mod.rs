#![allow(non_snake_case)]

use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::fmt::Debug;
use std::marker::PhantomData;
#[cfg(feature = "debug-output")]
use tracing::warn;

use crate::error::response_error::*;
use crate::error::SvcResponseError;

pub(crate) mod tasks_data;

pub(crate) struct SvcResponse<T: SvcRespTypeTrait + DeserializeOwned> {
    raw_resp: reqwest::Response,
    #[allow(non_snake_case)]
    __: PhantomData<T>,
}

impl<T: SvcRespTypeTrait + DeserializeOwned> SvcResponse<T> {
    pub(crate) fn new(raw_resp: reqwest::Response) -> Self {
        Self {
            raw_resp,
            __: PhantomData,
        }
    }

    pub(crate) async fn deserialize(self) -> Result<T, SvcResponseError> {
        let resp_str = self
            .raw_resp
            .text()
            .await
            .map_err(SvcResponseError::RespToStringError)?;

        #[cfg(feature = "debug-output")]
        warn!("Original response:\n'{}'", &resp_str);

        let resp =
            serde_json::from_str::<T>(&resp_str).map_err(SvcResponseError::SerializeError)?;

        #[cfg(feature = "debug-output")]
        warn!("Response as object:\n'{:?}'", resp);

        Ok(resp)
    }
}

pub(crate) trait SvcRespTypeTrait: Clone + Debug {
    type Result;

    fn get_task_result(&self) -> Self::Result;
}

#[derive(Deserialize, Debug)]
pub(crate) struct SvcResponseStruct<T: SvcRespTypeTrait> {
    errorId: u32,
    // https://zennolab.atlassian.net/wiki/spaces/APIS/pages/295396
    // https://zennolab.atlassian.net/wiki/spaces/APIS/pages/295310
    errorCode: Option<String>,
    errorDescription: Option<String>,
    #[serde(flatten)]
    flat_data: Option<T>,
}

impl<'a, T: SvcRespTypeTrait> SvcResponseStruct<T> {
    fn get_result(&'a self) -> Result<T, ResponseError> {
        if self.errorId == 0 {
            if let Some(r) = self.flat_data.clone() {
                Ok(r)
            } else {
                Err(ResponseError::SuccessResponseWithoutData)
            }
        } else {
            Err(ResponseError::Error)
        }
    }
}
