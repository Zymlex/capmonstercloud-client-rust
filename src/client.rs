use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client as HttpClient, StatusCode, Url};

use serde::de::DeserializeOwned;
use serde::Serialize;
#[cfg(feature = "debug-output")]
use tracing::warn;

use crate::error::ClientImplError::HttpClientCreationError;
use crate::error::TaskResultError::*;
use crate::error::*;
use crate::limits::{self, Limits, LimitsTrait};
use crate::options::Options;
use crate::requests::tasks_names::TaskReqTrait;
use crate::requests::{CreateTaskRequest, GetBalanceRequest, GetTaskResultRequest, make_svc_request, TaskData};
use crate::responses::tasks_data::{GetBalanceResp, GetTaskResultResp, TaskIdResp, TaskTypeTrait};
use crate::responses::{SvcRespTypeTrait, SvcResponse};
use crate::task::Task;
use crate::urls::Urls;

pub(crate) struct ClientImpl<'a> {
    pub(crate) options: Options<'a>,
    http_client: HttpClient,
    urls: Urls,
}

impl<'a> ClientImpl<'a> {
    pub(crate) fn new(
        ext_options: Options<'a>,
        http_client: Option<HttpClient>,
    ) -> Result<Self, ClientImplError> {
        let mut headers: HeaderMap = HeaderMap::with_capacity(1);
        headers.insert(
            "User-Agent",
            HeaderValue::from_static(concat!(
                "ZennoLab.CapMonsterCloud.RustClient/{}",
                env!("CARGO_PKG_VERSION")
            )),
        );

        let url = ext_options.service_uri.clone();

        Ok(Self {
            options: ext_options,
            http_client: http_client.unwrap_or(
                reqwest::ClientBuilder::new()
                    .default_headers(headers)
                    .timeout(limits::REQUEST_TIMEOUT)
                    .http2_keep_alive_interval(limits::HTTP2_KEEP_ALIVE_INTERVAL)
                    .http2_keep_alive_while_idle(false)
                    .build()
                    .map_err(HttpClientCreationError)?,
            ),
            urls: Urls::from(url),
        })
    }

    // https://zennolab.atlassian.net/wiki/spaces/APIS/pages/655432
    pub async fn get_balance_async(&self) -> Result<f64, GetBalanceError> {
        let body = serde_json::to_string(&GetBalanceRequest {
            clientKey: self.options.client_key,
        })
        .map_err(GetBalanceError::SerializeError)?;

        let resp = make_svc_request::<GetBalanceResp>(self.urls.get_balance_url(), body, &self.http_client)
            .await
            .map_err(GetBalanceError::RequestError)?;

        let resp_obj = resp
            .deserialize()
            .await
            .map_err(GetBalanceError::DeserializeError)?;

        Ok(resp_obj.balance)
    }

    pub(crate) async fn solve_impl<
        T: TaskReqTrait + Serialize,
        Y: TaskTypeTrait + DeserializeOwned + std::fmt::Debug,
    >(
        &self,
        data: T,
    ) -> Result<Y, SolveError>
    where
        Limits<T>: LimitsTrait,
    {
        let task = self
            .create_task(data)
            .await
            .map_err(SolveError::TaskCreationError)?;

        let res = self
            .get_task_result::<T, Y>(task)
            .await
            .map_err(SolveError::TaskResultError)?;

        Ok(res)
    }

    // https://zennolab.atlassian.net/wiki/spaces/APIS/pages/393308
    async fn create_task<T: TaskReqTrait + Serialize>(
        &self,
        data: T,
    ) -> Result<Task<T>, TaskCreationError>
    where
        Limits<T>: LimitsTrait,
    {
        let request_data = CreateTaskRequest::new(
            self.options.client_key,
            TaskData::new(data),
            self.options.soft_id,
        );
        
        let body = serde_json::to_string(&request_data).map_err(TaskCreationError::SerializeError)?;

        let resp =
            make_svc_request::<TaskIdResp>(self.urls.task_creation_url(), body, &self.http_client)
                .await
                .map_err(TaskCreationError::RequestError)?;

        let resp_obj = resp
            .deserialize()
            .await
            .map_err(TaskCreationError::DeserializeError)?;

        Ok(Task::new(resp_obj.taskId))
    }

    // https://zennolab.atlassian.net/wiki/spaces/APIS/pages/688194
    async fn get_task_result<
        T: TaskReqTrait,
        Y: TaskTypeTrait + DeserializeOwned + std::fmt::Debug,
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
        let body = serde_json::to_string(&request_data).map_err(TaskResultError::SerializeError)?;

        loop {
            let resp = make_svc_request::<GetTaskResultResp<Y>>(self.urls.get_task_result_url(), body.clone(), &self.http_client)
                .await
                .map_err(TaskResultError::RequestError)?;

            task.add_request_count();

            match resp.deserialize().await {
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

            task.check_and_wait_req_interval().await?;
        }
    }
}
