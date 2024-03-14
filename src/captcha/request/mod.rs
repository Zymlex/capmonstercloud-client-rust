#![allow(non_snake_case)]

use self::get_task_result_repeater::GetTaskResultRepeater;
use crate::cfg::{Config, limits};
use crate::error::RequestCreatorError;
use crate::error::RequestCreatorError::HttpClientCreationError;
use crate::{
    CreateTask, GetBalance, GetBalanceResp, GetTaskResult, GetTaskResultResp, MethodReqTrait,
    SvcErrorResp, SvcResp, SvcRespTypeTrait, SvcResponse, SvcSuccessResp, TaskIdResp, TaskReqTrait,
    TaskRespTrait,
};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client as HttpClient;
use reqwest::{StatusCode, Url};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::time::Duration;
use tracing::warn;
use crate::cfg::urls::Urls;

mod get_task_result_repeater;

pub(crate) struct RequestCreator<'a> {
    http_client: HttpClient,
    client_key: &'a str,
    urls: Urls,
}

impl<'a> RequestCreator<'a> {
    pub(crate) fn new(client_key: &'a str, urls: Urls) -> Result<Self, RequestCreatorError> {
        let mut headers: HeaderMap = HeaderMap::with_capacity(1);
        headers.insert(
            "User-Agent",
            HeaderValue::from_static(concat!(
                "ZennoLab.CapMonsterCloud.RustClient.",
                env!("CARGO_PKG_VERSION")
            )),
        );

        let http_client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .timeout(Duration::from_millis(limits::REQUEST_TIMEOUT_MS))
            .http2_keep_alive_interval(Duration::from_millis(limits::HTTP2_KEEP_ALIVE_INTERVAL_MS))
            .http2_keep_alive_while_idle(false)
            .build()
            .map_err(HttpClientCreationError)?;

        Ok(Self {
            http_client,
            client_key,
            urls,
        })
    }

    /// https://zennolab.atlassian.net/wiki/spaces/APIS/pages/393308
    pub(crate) async fn createTask<T: TaskReqTrait + Serialize>(
        &self,
        task_body: T,
    ) -> Result<SvcResponse<TaskIdResp>, RequestCreatorError> {
        let request_data = CreateTask {
            softId: Config::SOFT_ID,
            clientKey: self.client_key,
            task: task_body,
        };

        let resp_obj = self
            .make_svc_request::<CreateTask<'a, T>, TaskIdResp>(
                &self.urls.task_creation_url,
                &request_data,
            )
            .await?;

        Ok(resp_obj)
    }

    pub(crate) fn getTaskResultRepeater(&self, taskId: u32) -> GetTaskResultRepeater {
        GetTaskResultRepeater::new(taskId, self.client_key, self)
    }

    pub(crate) async fn getTaskResult<
        Y: TaskRespTrait + DeserializeOwned + std::fmt::Debug + 'a,
    >(
        &self,
        task_data: &GetTaskResult<'a>,
    ) -> Result<SvcResponse<GetTaskResultResp<Y>>, RequestCreatorError> {
        let result = self
            .make_svc_request::<GetTaskResult, GetTaskResultResp<Y>>(
                &self.urls.task_result_url,
                task_data,
            )
            .await?;

        Ok(result)
    }

    pub(crate) async fn getBalance(
        &self,
    ) -> Result<SvcResponse<GetBalanceResp>, RequestCreatorError> {
        let request_data = GetBalance {
            clientKey: self.client_key,
        };

        let result = self
            .make_svc_request::<GetBalance, GetBalanceResp>(&self.urls.balance_url, &request_data)
            .await?;

        Ok(result)
    }

    #[allow(clippy::needless_lifetimes)]
    async fn make_svc_request<
        T: MethodReqTrait + Serialize,
        X: SvcRespTypeTrait + DeserializeOwned + std::fmt::Debug + 'a,
    >(
        &self,
        url: &Url,
        request_data: &T,
    ) -> Result<SvcResponse<X>, RequestCreatorError> {
        #[cfg(feature = "debug-output")]
        warn!("Url:\n'{}'", url);

        let body = serde_json::to_string(&request_data)
            .map_err(RequestCreatorError::SerializationError)?;

        #[cfg(feature = "debug-output")]
        warn!("Body:\n'{}'", body);

        let raw_resp = self
            .http_client
            .post(url.clone())
            .body(body)
            .send()
            .await
            .map_err(RequestCreatorError::PostRequestError)?;

        let resp_status = raw_resp.status();

        if resp_status != StatusCode::OK {
            return Err(RequestCreatorError::NonSuccessResponseStatus(resp_status));
        }

        let resp_str = raw_resp
            .text()
            .await
            .map_err(RequestCreatorError::ResponseToStringError)?;

        #[cfg(feature = "debug-output")]
        warn!("Original response body:\n'{}'", &resp_str);

        let svc_struct = match serde_json::from_str::<SvcErrorResp>(&resp_str) {
            Ok(r) => SvcResp::Error(r),
            Err(ee) => {
                //TODO Need more precise check, but API is private
                if ee.is_data() {
                    // Trying to parse json as a task processing service response
                    // match serde_json::from_str::<SvcSuccessResp<TaskIdResp>>(&resp_str) {
                    //     Ok(r) => SvcResp::Success(r),
                    //     Err(_) => {
                    // Trying to parse json as a successful service response
                    match serde_json::from_str::<SvcSuccessResp<X>>(&resp_str) {
                        Ok(r) => SvcResp::Success(r),
                        Err(es) => {
                            return Err(RequestCreatorError::DeserializationSuccessError(es));
                        }
                    }
                    //     }
                    // }
                } else {
                    return Err(RequestCreatorError::DeserializationErrorError(ee));
                }
            }
        };

        #[cfg(feature = "debug-output")]
        warn!("Response as object:\n'{:?}'", &svc_struct);

        Ok(SvcResponse::new(
            svc_struct,
            #[cfg(feature = "keep-request-body")]
            resp_str,
        ))
    }
}
