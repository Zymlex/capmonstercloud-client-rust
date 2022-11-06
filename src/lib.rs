use std::time::Duration;

use reqwest::{Client, Url};
use serde::de::DeserializeOwned;
use tokio::time::sleep;

use crate::errors::{CapMonsterCloudClientError, TaskCreationError};
use crate::http_client::HttpClient;
use crate::options::ClientOptions;
use crate::requests::{
    CreateTaskRequest, GetBalanceRequest, GetTaskResultRequest, TaskData, TaskReqTrait,
};
use crate::requests::tasks_data::{
    FunCaptchaTaskProxylessReq, FunCaptchaTaskReq, GeeTestTaskProxylessReq, GeeTestTaskReq,
    HCaptchaTaskProxylessReq, HCaptchaTaskReq, ImageToTextTaskReq, NoCaptchaTaskProxylessReq,
    RecaptchaV2EnterpriseTaskProxylessReq, RecaptchaV2EnterpriseTaskReq,
    RecaptchaV3TaskProxylessReq,
};
use crate::responses::Response;
use crate::responses::tasks_data::{
    FunCaptchaTaskProxylessResp, FunCaptchaTaskResp, GeeTestTaskProxylessResp, GeeTestTaskResp,
    GetBalanceResp, GetTaskResultResp, HCaptchaTaskProxylessResp, HCaptchaTaskResp,
    ImageToTextTaskResp, NoCaptchaTaskProxylessResp, RecaptchaV2EnterpriseTaskProxylessResp,
    RecaptchaV2EnterpriseTaskResp, RecaptchaV3TaskProxylessResp, TaskIdResp, TaskRespTrait,
};
use crate::task::Task;

mod errors;
mod http_client;
mod modules;
mod options;
mod proxy_type;
mod requests;
mod responses;
mod task;
mod tests;

pub struct CapMonsterCloudClient<'a> {
    http_client: HttpClient,
    client_options: ClientOptions<'a>,
}

impl<'a> CapMonsterCloudClient<'a> {
    pub fn new(client_key: &'a str) -> Result<Self, CapMonsterCloudClientError> {
        let options = ClientOptions::new(client_key).unwrap();
        
        Self::new_ex(options, None)
    }
    
    pub fn new_ex(
        client_options: ClientOptions<'a>,
        http_client: Option<Client>,
    ) -> Result<Self, CapMonsterCloudClientError> {
        Ok(Self {
            http_client: HttpClient::new(http_client),
            client_options,
        })
    }
    
    pub async fn get_balance_async(&self) -> Result<f64, CapMonsterCloudClientError> {
        let url = self.get_service_url_with_path("getBalance").await;
        
        let body = serde_json::to_string(&GetBalanceRequest {
            clientKey: self.client_options.client_key,
        })
            .unwrap();
        
        let response = self.http_client.post(url, body).await.unwrap();
        
        let response: Response<GetBalanceResp> = response.json().await.unwrap();
        
        Ok(response.get_result().unwrap().balance)
    }
    
    pub async fn image_to_text_task(
        &self,
        data: ImageToTextTaskReq<'a>,
    ) -> Result<ImageToTextTaskResp, TaskCreationError> {
        self.solve_impl("ImageToTextTask", data).await
    }
    
    pub async fn no_captcha_task_proxyless(
        &self,
        data: NoCaptchaTaskProxylessReq<'a>,
    ) -> Result<NoCaptchaTaskProxylessResp, TaskCreationError> {
        self.solve_impl("NoCaptchaTaskProxyless", data).await
    }
    
    pub async fn no_captcha_task(
        &self,
        data: NoCaptchaTaskProxylessReq<'a>,
    ) -> Result<NoCaptchaTaskProxylessResp, TaskCreationError> {
        self.solve_impl("NoCaptchaTask", data).await
    }
    
    pub async fn recaptcha_v3_task_proxyless(
        &self,
        data: RecaptchaV3TaskProxylessReq<'a>,
    ) -> Result<RecaptchaV3TaskProxylessResp, TaskCreationError> {
        self.solve_impl("RecaptchaV3TaskProxyless", data).await
    }
    
    pub async fn recaptcha_v2_enterprise_task(
        &self,
        data: RecaptchaV2EnterpriseTaskReq<'a>,
    ) -> Result<RecaptchaV2EnterpriseTaskResp, TaskCreationError> {
        self.solve_impl("RecaptchaV2EnterpriseTask", data).await
    }
    
    pub async fn recaptcha_v2_enterprise_task_proxyless(
        &self,
        data: RecaptchaV2EnterpriseTaskProxylessReq<'a>,
    ) -> Result<RecaptchaV2EnterpriseTaskProxylessResp, TaskCreationError> {
        self.solve_impl("RecaptchaV2EnterpriseTaskProxyless", data)
            .await
    }
    
    pub async fn funcaptcha_task(
        &self,
        data: FunCaptchaTaskReq<'a>,
    ) -> Result<FunCaptchaTaskResp, TaskCreationError> {
        self.solve_impl("FunCaptchaTask", data).await
    }
    
    pub async fn funcaptcha_task_proxyless(
        &self,
        data: FunCaptchaTaskProxylessReq<'a>,
    ) -> Result<FunCaptchaTaskProxylessResp, TaskCreationError> {
        self.solve_impl("FunCaptchaTaskProxyless", data).await
    }
    
    pub async fn hcaptcha_task(
        &self,
        data: HCaptchaTaskReq<'a>,
    ) -> Result<HCaptchaTaskResp, TaskCreationError> {
        self.solve_impl("HCaptchaTask", data).await
    }
    
    pub async fn hcaptcha_task_proxyless(
        &self,
        data: HCaptchaTaskProxylessReq<'a>,
    ) -> Result<HCaptchaTaskProxylessResp, TaskCreationError> {
        self.solve_impl("HCaptchaTaskProxyless", data).await
    }
    
    pub async fn geetest_task(
        &self,
        data: GeeTestTaskReq<'a>,
    ) -> Result<GeeTestTaskResp, TaskCreationError> {
        self.solve_impl("GeeTestTask", data).await
    }
    
    pub async fn geetest_task_proxyless(
        &self,
        data: GeeTestTaskProxylessReq<'a>,
    ) -> Result<GeeTestTaskProxylessResp, TaskCreationError> {
        self.solve_impl("GeeTestTaskProxyless", data).await
    }
    
    // pub(crate) async fn solve(&self, task_type: TaskType<'a>) -> Result<(), TaskCreationError> {
    //     match task_type {
    //         TaskType::ImageToTextTask(data) => {
    //             self.solve_impl::<ImageToTextTaskReq, ImageToTextTaskResp>("ImageToTextTask", data)
    //         }
    //         TaskType::NoCaptchaTaskProxyless(data) => {
    //             self.solve_impl::<NoCaptchaTaskProxylessReq, NoCaptchaTaskProxylessResp>("NoCaptchaTaskProxyless", data)
    //         }
    //         TaskType::NoCaptchaTask(data) => {
    //             self.solve_impl::<NoCaptchaTaskReq, NoCaptchaTaskResp>("NoCaptchaTask", data)
    //         }
    //         TaskType::RecaptchaV3TaskProxyless(data) => {
    //             self.solve_impl::<RecaptchaV3TaskProxylessReq, RecaptchaV3TaskProxylessResp>("RecaptchaV3TaskProxyless", data)
    //         }
    //         TaskType::RecaptchaV2EnterpriseTask(data) => {
    //             self.solve_impl::<RecaptchaV2EnterpriseTaskReq, RecaptchaV2EnterpriseTaskResp>("RecaptchaV2EnterpriseTask", data)
    //         }
    //         TaskType::RecaptchaV2EnterpriseTaskProxyless(data) => {
    //             self.solve_impl::<RecaptchaV2EnterpriseTaskProxylessReq, RecaptchaV2EnterpriseTaskProxylessResp>("RecaptchaV2EnterpriseTaskProxyless", data)
    //         }
    //         TaskType::FunCaptchaTask(data) => {
    //             self.solve_impl::<FunCaptchaTaskReq, FunCaptchaTaskResp>("FunCaptchaTask", data)
    //         }
    //         TaskType::FunCaptchaTaskProxyless(data) => {
    //             self.solve_impl::<FunCaptchaTaskProxylessReq, FunCaptchaTaskProxylessResp>("FunCaptchaTaskProxyless", data)
    //         }
    //         TaskType::HCaptchaTask(data) => {
    //             self.solve_impl::<HCaptchaTaskReq, HCaptchaTaskResp>("HCaptchaTask", data)
    //         }
    //         TaskType::HCaptchaTaskProxyless(data) => {
    //             self.solve_impl::<HCaptchaTaskProxylessReq, HCaptchaTaskProxylessResp>("HCaptchaTaskProxyless", data)
    //         }
    //         TaskType::GeeTestTask(data) => {
    //             self.solve_impl::<GeeTestTaskReq, GeeTestTaskResp>("GeeTestTask", data)
    //         }
    //         TaskType::GeeTestTaskProxyless(data) => {
    //             self.solve_impl::<GeeTestTaskProxylessReq, GeeTestTaskProxylessResp>("GeeTestTaskProxyless", data)
    //         }
    //     }.await
    // }
    
    async fn solve_impl<T: TaskReqTrait, Y: TaskRespTrait + DeserializeOwned>(
        &self,
        name: &str,
        data: T,
    ) -> Result<Y, TaskCreationError> {
        let url = self.get_service_url_with_path("createTask").await;
        
        let request_data = CreateTaskRequest {
            clientKey: self.client_options.client_key,
            task: TaskData::new(name, data),
            // callbackUrl: self.client_options.callback_url,
        };
        let body = serde_json::to_string(&request_data).unwrap();
        
        let raw_response = self.http_client.post(url, body).await.unwrap();
        
        let response: Response<TaskIdResp> = raw_response.json().await.unwrap();
        
        let task_id = response.get_result().unwrap().taskId;
        
        // Task check
        
        let mut task = Task::new();
        
        let url = self.get_service_url_with_path("getTaskResult").await;
        
        let request_data = GetTaskResultRequest {
            clientKey: self.client_options.client_key,
            taskId: task_id,
        };
        let body = serde_json::to_string(&request_data).unwrap();
        
        loop {
            sleep(Duration::from_secs(21)).await;
            
            task.add_count();
            
            let raw_response = self
                .http_client
                .post(url.clone(), body.clone())
                .await
                .unwrap();
            
            let response = raw_response
                .json::<Response<GetTaskResultResp<Y>>>()
                .await
                .unwrap();
            
            if let Ok(r) = response.get_result() {
                return Ok(r.get_task_result().unwrap());
            }
            
            task.check_state().unwrap();
        }
    }
    
    async fn get_service_url_with_path(&self, url_path: &str) -> Url {
        let mut url = self.client_options.service_uri.clone();
        url.set_path(url_path);
        url
    }
}
