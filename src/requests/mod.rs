#![allow(non_snake_case)]

use crate::error::SvcRequestError;
use crate::requests::tasks_names::TaskReqTrait;
use crate::responses::tasks_data::TaskTypeTrait;
use crate::responses::{SvcRespTypeTrait, SvcResponse};
use reqwest::{Client, StatusCode, Url};
use serde::de::DeserializeOwned;
use serde::Serialize;
use tracing::warn;

pub(crate) mod tasks_data;
pub(crate) mod tasks_names;

#[allow(clippy::needless_lifetimes)]
pub(crate) async fn make_svc_request<'a, T: SvcRespTypeTrait + DeserializeOwned>(
    url: Url,
    body: String,
    http_client: &Client,
) -> Result<SvcResponse<T>, SvcRequestError> {    
    #[cfg(feature = "debug-output")]
    warn!("Body:\n'{}'", body);
    
    let raw_resp = http_client
        .post(url)
        .body(body)
        .send()
        .await
        .map_err(SvcRequestError::PostRequestError)?;
    
    let resp_status = raw_resp.status();
    
    if resp_status != StatusCode::OK {
        return Err(SvcRequestError::NonSuccessRespStatus(resp_status));
    }
    
    Ok(SvcResponse::new(raw_resp))
}

#[derive(Serialize)]
pub(crate) struct CreateTaskRequest<'a, T: TaskReqTrait> {
    clientKey: &'a str,
    task: TaskData<'a, T>,
    // pub(crate) callbackUrl: Option<&'a str>,
    softId: u32,
}

impl<'a, T: TaskReqTrait> CreateTaskRequest<'a, T> {
    pub(crate) fn new(
        clientKey: &'a str,
        task: TaskData<'a, T>,
        softId: u32,
    ) -> CreateTaskRequest<'a, T> {
        Self {
            clientKey,
            task,
            softId,
        }
    }
}

#[derive(Serialize)]
pub(crate) struct TaskData<'a, T: TaskReqTrait> {
    #[serde(rename = "type")]
    typ: &'a str,
    #[serde(flatten)]
    flat_data: T,
}

impl<'a, T: TaskReqTrait> TaskData<'a, T> {
    pub(crate) fn new(flatten: T) -> Self {
        Self {
            typ: flatten.get_task_name(),
            flat_data: flatten,
        }
    }
}

#[derive(Serialize)]
pub(crate) struct GetBalanceRequest<'a> {
    pub(crate) clientKey: &'a str,
}

#[derive(Serialize)]
pub(crate) struct GetTaskResultRequest<'a> {
    pub(crate) clientKey: &'a str,
    pub(crate) taskId: u32,
}
