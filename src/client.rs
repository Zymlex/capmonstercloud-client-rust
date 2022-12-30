use reqwest::{Client as HttpClient, Url};
use std::time::Duration;

use serde::de::DeserializeOwned;
use tokio::time::sleep;
#[cfg(feature = "debug-output")]
use tracing::warn;

use crate::error::GetBalanceError::*;
use crate::error::TaskResultError::*;
use crate::error::*;
use crate::limits;
use crate::limits::{Limits, LimitsTrait};
use crate::options::Options;
use crate::requests::{
    CreateTaskRequest, GetBalanceRequest, GetTaskResultRequest, TaskData, TaskReqTrait,
};
use crate::responses::tasks_data::{GetBalanceResp, GetTaskResultResp, TaskIdResp, TaskRespTrait};
use crate::responses::{RespDataTrait, Response};
use crate::task::Task;
use crate::urls::Urls;

pub(crate) struct ClientImpl<'a> {
    pub(crate) options: Options<'a>,
    http_client: HttpClient,
    urls: Urls,
}

impl<'a> ClientImpl<'a> {
    pub(crate) fn new(ext_options: Options<'a>, http_client: HttpClient) -> Self {
        let url = ext_options.service_uri.clone();

        Self {
            options: ext_options,
            http_client,
            urls: Urls::from(url),
        }
    }

    pub async fn get_balance_async(&self) -> Result<f64, GetBalanceError> {
        let body = serde_json::to_string(&GetBalanceRequest {
            clientKey: self.options.client_key,
        })
        .map_err(GetBalanceError::StructToJsonError)?;

        let resp_obj = self
            .send_request::<Response<GetBalanceResp>>(self.urls.get_balance_url(), body)
            .await
            .map_err(GetBalanceError::RequestError)?;

        Ok(resp_obj.get_result().map_err(GetResultError)?.balance)
    }

    pub(crate) async fn solve_impl<
        T: TaskReqTrait,
        Y: TaskRespTrait + DeserializeOwned + std::fmt::Debug,
    >(
        &self,
        name: &str,
        data: T,
    ) -> Result<Y, SolveError>
    where
        Limits<T>: LimitsTrait,
    {
        let task = self.create_task(name, data).await.map_err(SolveError::TaskCreationError)?;

        let res = self.get_task_result::<T, Y>(task).await.map_err(SolveError::TaskResultError)?;

        Ok(res)
    }

    async fn create_task<T: TaskReqTrait>(
        &self,
        name: &str,
        data: T,
    ) -> Result<Task<T>, TaskCreationError>
    where
        Limits<T>: LimitsTrait,
    {
        let request_data = CreateTaskRequest::new(
            self.options.client_key,
            TaskData::new(name, data),
            self.options.soft_id,
        );
        let body =
            serde_json::to_string(&request_data).map_err(TaskCreationError::StructToJsonError)?;

        let resp_obj = self
            .send_request::<Response<TaskIdResp>>(self.urls.create_task_url(), body)
            .await
            .map_err(TaskCreationError::RequestError)?;

        let task_id = resp_obj
            .get_result()
            .map_err(TaskCreationError::InvalidResponse)?
            .taskId;

        Ok(Task::new(task_id))
    }

    async fn get_task_result<
        T: TaskReqTrait,
        Y: TaskRespTrait + DeserializeOwned + std::fmt::Debug,
    >(
        &self,
        mut task: Task<T>,
    ) -> Result<Y, TaskResultError>
    where
        Limits<T>: LimitsTrait,
    {
        let request_data = GetTaskResultRequest {
            clientKey: self.options.client_key,
            taskId: task.task_id,
        };
        let body = serde_json::to_string(&request_data).map_err(TaskResultError::StructToJsonError)?;

        loop {
            sleep(task.request_interval()).await;

            let resp_obj = self
                .send_request::<Response<GetTaskResultResp<Y>>>(
                    self.urls.get_task_result_url(),
                    body.clone(),
                )
                .await
                .map_err(TaskResultError::RequestError)?;

            task.add_request_count();

            match resp_obj.get_result() {
                Ok(r) => match r.get_task_result() {
                    Ok(r) => return Ok(r),
                    Err(e) => match e {
                        GetTaskError::Processing => continue,
                        GetTaskError::ReadyTaskWithoutSolution => {
                            return Err(InvalidResponse("ReadyTaskWithoutSolution"))
                        }
                    },
                },
                Err(e) => {}
            }

            task.check_before_wait()?;
        }
    }

    async fn send_request<T: DeserializeOwned + std::fmt::Debug>(
        &self,
        url: Url,
        body: String,
    ) -> Result<T, HttpClientError> {
        #[cfg(feature = "debug-output")]
        warn!("Body:\n'{}'", body);

        let raw_resp = self
            .http_client
            .post(url)
            .body(body)
            .timeout(limits::REQUEST_TIMEOUT)
            .send()
            .await
            .map_err(HttpClientError::PostRequestError)?;

        let resp_str = raw_resp
            .text()
            .await
            .map_err(HttpClientError::RawRespToTextError)?;

        #[cfg(feature = "debug-output")]
        warn!("Original response:\n'{}'", &resp_str);

        let resp =
            serde_json::from_str::<T>(&resp_str).map_err(HttpClientError::TextToStructError)?;

        #[cfg(feature = "debug-output")]
        warn!("Response as object:\n'{:?}'", resp);

        Ok(resp)
    }
}
