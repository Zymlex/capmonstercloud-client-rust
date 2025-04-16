#![allow(non_snake_case, reason = "API")]

use crate::cfg::limits;
use crate::cfg::urls::Urls;
use crate::error::RequestCreatorError;
use crate::error::RequestCreatorError::HttpClientCreation;
use crate::*;
use reqwest::Client as HttpClient;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{StatusCode, Url};
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::time::Duration;
#[cfg(feature = "debug-output")]
use tracing::info;

pub(crate) struct RequestCreator<'a> {
    http_client: HttpClient,
    urls: Urls,
    soft_id: &'a str,
    client_key: &'a str,
}

impl<'a> RequestCreator<'a> {
    pub(crate) fn new(
        urls: Urls,
        soft_id: &'a str,
        client_key: &'a str,
    ) -> Result<Self, RequestCreatorError> {
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
            .gzip(true)
            // .timeout(Duration::from_millis(limits::REQUEST_TIMEOUT_MS))
            .http2_keep_alive_interval(Duration::from_millis(limits::HTTP2_KEEP_ALIVE_INTERVAL_MS))
            .http2_keep_alive_while_idle(false)
            .build()
            .map_err(HttpClientCreation)?;

        Ok(Self {
            http_client,
            urls,
            soft_id,
            client_key,
        })
    }

    pub(crate) async fn getUserAgent(&self) -> Result<String, RequestCreatorError> {
        let url = self.urls.user_agent.clone();

        #[cfg(feature = "debug-output")]
        info!("Url:\n{}", url);

        let raw_resp = self
            .http_client
            .get(url)
            .send()
            .await
            .map_err(RequestCreatorError::GetRequest)?;

        let resp_status = raw_resp.status();

        if resp_status != StatusCode::OK {
            return Err(RequestCreatorError::NonSuccessResponseStatus(resp_status));
        }

        let resp_str = raw_resp
            .text()
            .await
            .map_err(RequestCreatorError::ResponseToString)?;

        #[cfg(feature = "debug-output")]
        info!("Original response body:\n{}", &resp_str);

        Ok(resp_str.into())
    }

    pub(crate) async fn getBalance(
        &self,
    ) -> Result<SvcResponse<GetBalanceResp>, RequestCreatorError> {
        let request_data = GetBalance {
            clientKey: self.client_key,
        };

        println!("{:?}", &self.urls.balance);

        let result = self
            .make_svc_post_request::<GetBalance, GetBalanceResp>(&self.urls.balance, &request_data)
            .await?;

        Ok(result)
    }

    pub(crate) async fn createTask<T: TaskReqTrait + Serialize + Send + Sync>(
        &self,
        task_body: T,
    ) -> Result<SvcResponse<TaskIdResp>, RequestCreatorError> {
        let request_data = CreateTask {
            softId: self.soft_id,
            clientKey: self.client_key,
            task: task_body,
        };

        let resp_obj = self
            .make_svc_post_request::<CreateTask<'a, T>, TaskIdResp>(
                &self.urls.task_creation,
                &request_data,
            )
            .await?;

        Ok(resp_obj)
    }

    pub(crate) async fn getTaskResult<
        Y: TaskRespTrait + DeserializeOwned + std::fmt::Debug + 'a,
    >(
        &self,
        taskId: u32,
    ) -> Result<SvcResponse<GetTaskResultResp<Y>>, RequestCreatorError> {
        let result = self
            .make_svc_post_request::<GetTaskResult, GetTaskResultResp<Y>>(
                &self.urls.task_result,
                &GetTaskResult {
                    clientKey: self.client_key,
                    taskId,
                },
            )
            .await?;

        Ok(result)
    }

    async fn make_svc_post_request<
        T: MethodReqTrait + Serialize,
        X: SvcRespTypeTrait + DeserializeOwned + std::fmt::Debug + 'a,
    >(
        &'a self,
        url: &'a Url,
        request_data: &'a T,
    ) -> Result<SvcResponse<X>, RequestCreatorError> {
        #[cfg(feature = "debug-output")]
        info!("Url:\n{}", url);

        let body =
            serde_json::to_string(&request_data).map_err(RequestCreatorError::Serialization)?;

        #[cfg(feature = "debug-output")]
        info!("Body:\n{}", body);

        let raw_resp = self
            .http_client
            .post(url.clone())
            .body(body)
            .send()
            .await
            .map_err(RequestCreatorError::PostRequest)?;

        let resp_status = raw_resp.status();

        if resp_status != StatusCode::OK {
            return Err(RequestCreatorError::NonSuccessResponseStatus(resp_status));
        }

        let resp_str = raw_resp
            .text()
            .await
            .map_err(RequestCreatorError::ResponseToString)?;

        #[cfg(feature = "debug-output")]
        info!("Original response body:\n{}", &resp_str);

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
                            return Err(RequestCreatorError::DeserializationSvcSuccess(es));
                        }
                    }
                    //     }
                    // }
                } else {
                    return Err(RequestCreatorError::DeserializationSvcError(ee));
                }
            }
        };

        #[cfg(feature = "debug-output")]
        info!("Response as object:\n{:?}", &svc_struct);

        Ok(SvcResponse::new(svc_struct))
    }
}
