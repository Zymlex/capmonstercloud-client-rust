use reqwest::Client as HttpClient;
#[cfg(feature = "debug-output")]
use tracing::Level;

use crate::client::ClientImpl;
use crate::error::*;
use crate::options::Options;
use crate::requests::tasks_data::*;
use crate::responses::tasks_data::*;

mod client;
mod modules;
mod options;
mod proxy_type;
mod requests;
mod responses;
mod task;
mod tests;
mod urls;
mod error;
mod limits;

pub struct CapMonsterCloudClient<'a> {
    client: ClientImpl<'a>,
}

impl<'a> CapMonsterCloudClient<'a> {
    pub fn new(client_key: &'a str) -> Result<Self, CapMonsterCloudClientError> {
        
        #[cfg(feature = "debug-output")]
        tracing_subscriber::fmt().with_max_level(Level::TRACE).init();
        
        let options = Options::new(client_key)?;

        Self::new_ex(options, None)
    }

    pub fn new_ex(
        client_options: Options<'a>,
        http_client: Option<HttpClient>,
    ) -> Result<Self, CapMonsterCloudClientError> {
        Ok(Self {
            client: ClientImpl::new(client_options, http_client.unwrap_or_default()),
        })
    }

    pub async fn get_balance_async(&self) -> Result<f64, GetBalanceError> {
        self.client.get_balance_async().await
    }

    pub async fn image_to_text_task(
        &self,
        data: ImageToTextTaskReq<'a>,
    ) -> Result<ImageToTextTaskResp, SolveError> {
        self.client.solve_impl("ImageToTextTask", data).await
    }
    
    pub async fn no_captcha_task_proxyless(
        &self,
        data: NoCaptchaTaskProxylessReq<'a>,
    ) -> Result<NoCaptchaTaskProxylessResp, SolveError> {
        self.client.solve_impl("NoCaptchaTaskProxyless", data).await
    }
    
    pub async fn no_captcha_task(
        &self,
        data: NoCaptchaTaskReq<'a>,
    ) -> Result<NoCaptchaTaskResp, SolveError> {
        self.client.solve_impl("NoCaptchaTask", data).await
    }
    
    pub async fn recaptcha_v3_task_proxyless(
        &self,
        data: RecaptchaV3TaskProxylessReq<'a>,
    ) -> Result<RecaptchaV3TaskProxylessResp, SolveError> {
        self.client
            .solve_impl("RecaptchaV3TaskProxyless", data)
            .await
    }
    
    pub async fn recaptcha_v2_enterprise_task(
        &self,
        data: RecaptchaV2EnterpriseTaskReq<'a>,
    ) -> Result<RecaptchaV2EnterpriseTaskResp, SolveError> {
        self.client
            .solve_impl("RecaptchaV2EnterpriseTask", data)
            .await
    }
    
    pub async fn recaptcha_v2_enterprise_task_proxyless(
        &self,
        data: RecaptchaV2EnterpriseTaskProxylessReq<'a>,
    ) -> Result<RecaptchaV2EnterpriseTaskProxylessResp, SolveError> {
        self.client
            .solve_impl("RecaptchaV2EnterpriseTaskProxyless", data)
            .await
    }
    
    pub async fn funcaptcha_task(
        &self,
        data: FunCaptchaTaskReq<'a>,
    ) -> Result<FunCaptchaTaskResp, SolveError> {
        self.client.solve_impl("FunCaptchaTask", data).await
    }
    
    pub async fn funcaptcha_task_proxyless(
        &self,
        data: FunCaptchaTaskProxylessReq<'a>,
    ) -> Result<FunCaptchaTaskProxylessResp, SolveError> {
        self.client
            .solve_impl("FunCaptchaTaskProxyless", data)
            .await
    }
    
    pub async fn hcaptcha_task(
        &self,
        data: HCaptchaTaskReq<'a>,
    ) -> Result<HCaptchaTaskResp, SolveError> {
        self.client.solve_impl("HCaptchaTask", data).await
    }
    
    pub async fn hcaptcha_task_proxyless(
        &self,
        data: HCaptchaTaskProxylessReq<'a>,
    ) -> Result<HCaptchaTaskProxylessResp, SolveError> {
        self.client.solve_impl("HCaptchaTaskProxyless", data).await
    }
    
    pub async fn geetest_task(
        &self,
        data: GeeTestTaskReq<'a>,
    ) -> Result<GeeTestTaskResp, SolveError> {
        self.client.solve_impl("GeeTestTask", data).await
    }
    
    pub async fn geetest_task_proxyless(
        &self,
        data: GeeTestTaskProxylessReq<'a>,
    ) -> Result<GeeTestTaskProxylessResp, SolveError> {
        self.client.solve_impl("GeeTestTaskProxyless", data).await
    }

    // pub(crate) async fn solve<T: ResponseTrait>(&self, task_type: TaskType<'a>) -> Result<T, TaskCreationError> {
    //     Ok(match task_type {
    //         TaskType::ImageToTextTask(data) => {
    //             self.client.solve_impl::<ImageToTextTaskReq, ImageToTextTaskResp>("ImageToTextTask", data)
    //         }
    //         TaskType::NoCaptchaTaskProxyless(data) => {
    //             self.client.solve_impl::<NoCaptchaTaskProxylessReq, NoCaptchaTaskProxylessResp>("NoCaptchaTaskProxyless", data)
    //         }
    //         TaskType::NoCaptchaTask(data) => {
    //             self.client.solve_impl::<NoCaptchaTaskReq, NoCaptchaTaskResp>("NoCaptchaTask", data)
    //         }
    //         TaskType::RecaptchaV3TaskProxyless(data) => {
    //             self.client.solve_impl::<RecaptchaV3TaskProxylessReq, RecaptchaV3TaskProxylessResp>("RecaptchaV3TaskProxyless", data)
    //         }
    //         TaskType::RecaptchaV2EnterpriseTask(data) => {
    //             self.client.solve_impl::<RecaptchaV2EnterpriseTaskReq, RecaptchaV2EnterpriseTaskResp>("RecaptchaV2EnterpriseTask", data)
    //         }
    //         TaskType::RecaptchaV2EnterpriseTaskProxyless(data) => {
    //             self.client.solve_impl::<RecaptchaV2EnterpriseTaskProxylessReq, RecaptchaV2EnterpriseTaskProxylessResp>("RecaptchaV2EnterpriseTaskProxyless", data)
    //         }
    //         TaskType::FunCaptchaTask(data) => {
    //             self.client.solve_impl::<FunCaptchaTaskReq, FunCaptchaTaskResp>("FunCaptchaTask", data)
    //         }
    //         TaskType::FunCaptchaTaskProxyless(data) => {
    //             self.client.solve_impl::<FunCaptchaTaskProxylessReq, FunCaptchaTaskProxylessResp>("FunCaptchaTaskProxyless", data)
    //         }
    //         TaskType::HCaptchaTask(data) => {
    //             self.client.solve_impl::<HCaptchaTaskReq, HCaptchaTaskResp>("HCaptchaTask", data)
    //         }
    //         TaskType::HCaptchaTaskProxyless(data) => {
    //             self.client.solve_impl::<HCaptchaTaskProxylessReq, HCaptchaTaskProxylessResp>("HCaptchaTaskProxyless", data)
    //         }
    //         TaskType::GeeTestTask(data) => {
    //             self.client.solve_impl::<GeeTestTaskReq, GeeTestTaskResp>("GeeTestTask", data)
    //         }
    //         TaskType::GeeTestTaskProxyless(data) => {
    //             self.client.solve_impl::<GeeTestTaskProxylessReq, GeeTestTaskProxylessResp>("GeeTestTaskProxyless", data)
    //         }
    //     }.await?
    // }
}
