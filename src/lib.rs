#![cfg_attr(not(debug_assertions), deny(allow))]
#![cfg_attr(not(debug_assertions), deny(warnings))]
#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::allow_attributes_without_reason
)]
#![deny(nonstandard_style)]
#![forbid(non_ascii_idents, uncommon_codepoints, future_incompatible)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::redundant_pub_crate)]

pub use self::captcha::*;

use self::cfg::Config;
use self::error::{
    CapMonsterCloudClientError::{self, ClientImplError},
    GetBalanceError, SolveError,
};
use self::taskmgr::Solver;

mod captcha;
mod cfg;
mod error;
mod taskmgr;
mod tests;

/// Call [`new`][CapMonsterCloudClient::new] to instantiate this struct.
pub struct CapMonsterCloudClient<'a> {
    taskmgr: Solver<'a>,
}

impl<'a> CapMonsterCloudClient<'a> {
    /// Creates new capmonster.cloud сlient
    pub fn new(client_key: &'a str) -> Result<Self, CapMonsterCloudClientError> {
        Ok(Self {
            taskmgr: Solver::new(Config::new(client_key)?).map_err(ClientImplError)?,
        })
    }

    // /// Creates new capmonster.cloud сlient with additional options
    // fn new_ex(cfg: Config<'a>) -> Result<Self, CapMonsterCloudClientError> {
    //     Ok(Self {
    //         taskmgr: Solver::new(cfg).map_err(ClientImplError)?,
    //     })
    // }

    /// Get the amount of funds on the account.
    ///
    /// [Doc](https://docs.capmonster.cloud/docs/api/methods/get-balance)
    pub async fn get_balance_async(&self) -> Result<SvcResponse<GetBalanceResp>, GetBalanceError> {
        self.taskmgr.get_balance_async().await
    }

    /// [Doc](https://docs.capmonster.cloud/docs/captchas/image-to-text)
    pub async fn image_to_text_task(
        &self,
        data: ImageToTextTask<'a>,
    ) -> Result<SvcResponse<GetTaskResultResp<ImageToTextTaskResp>>, SolveError> {
        self.taskmgr.solve_impl(data).await
    }

    /// [Doc](https://docs.capmonster.cloud/docs/captchas/no-captcha-task)
    pub async fn recaptcha_v2_task_proxyless(
        &self,
        data: RecaptchaV2TaskProxyless<'a>,
    ) -> Result<SvcResponse<GetTaskResultResp<RecaptchaV2TaskProxylessResp>>, SolveError> {
        self.taskmgr.solve_impl(data).await
    }

    /// [Doc](https://docs.capmonster.cloud/docs/captchas/no-captcha-task)
    pub async fn recaptcha_v2_task(
        &self,
        data: RecaptchaV2Task<'a>,
    ) -> Result<SvcResponse<GetTaskResultResp<RecaptchaV2TaskResp>>, SolveError> {
        self.taskmgr.solve_impl(data).await
    }

    /// [Doc](https://docs.capmonster.cloud/docs/captchas/recaptcha-v3-task)
    pub async fn recaptcha_v3_task_proxyless(
        &self,
        data: RecaptchaV3TaskProxyless<'a>,
    ) -> Result<SvcResponse<GetTaskResultResp<RecaptchaV3TaskProxylessResp>>, SolveError> {
        self.taskmgr.solve_impl(data).await
    }

    /// [Doc](https://docs.capmonster.cloud/docs/captchas/recaptcha-v2-enterprise-task)
    pub async fn recaptcha_v2_enterprise_task_proxyless(
        &self,
        data: RecaptchaV2EnterpriseTaskProxyless<'a>,
    ) -> Result<SvcResponse<GetTaskResultResp<RecaptchaV2EnterpriseTaskProxylessResp>>, SolveError>
    {
        self.taskmgr.solve_impl(data).await
    }

    /// [Doc](https://docs.capmonster.cloud/docs/captchas/recaptcha-v2-enterprise-task)
    pub async fn recaptcha_v2_enterprise_task(
        &self,
        data: RecaptchaV2EnterpriseTask<'a>,
    ) -> Result<SvcResponse<GetTaskResultResp<RecaptchaV2EnterpriseTaskResp>>, SolveError> {
        self.taskmgr.solve_impl(data).await
    }

    // /// [Doc](???)
    // pub async fn funcaptcha_task(
    //     &self,
    //     data: FunCaptchaTask<'a>,
    // ) -> Result<SvcResponse<GetTaskResultResp<FunCaptchaTaskResp>>, SolveError> {
    //     self.taskmgr.solve_impl(data).await
    // }
    // 
    // /// [Doc](???)
    // pub async fn funcaptcha_task_proxyless(
    //     &self,
    //     data: FunCaptchaTaskProxyless<'a>,
    // ) -> Result<SvcResponse<GetTaskResultResp<FunCaptchaTaskProxylessResp>>, SolveError> {
    //     self.taskmgr.solve_impl(data).await
    // }

    /// [Doc](https://docs.capmonster.cloud/docs/captchas/hcaptcha-task)
    pub async fn hcaptcha_task(
        &self,
        data: HCaptchaTask<'a>,
    ) -> Result<SvcResponse<GetTaskResultResp<HCaptchaTaskResp>>, SolveError> {
        self.taskmgr.solve_impl(data).await
    }

    /// [Doc](https://docs.capmonster.cloud/docs/captchas/hcaptcha-task)
    pub async fn hcaptcha_task_proxyless(
        &self,
        data: HCaptchaTaskProxyless<'a>,
    ) -> Result<SvcResponse<GetTaskResultResp<HCaptchaTaskProxylessResp>>, SolveError> {
        self.taskmgr.solve_impl(data).await
    }

    /// [Doc](https://docs.capmonster.cloud/docs/captchas/geetest-task)
    pub async fn geetest_task(
        &self,
        data: GeeTestTask<'a>,
    ) -> Result<SvcResponse<GetTaskResultResp<GeeTestTaskResp>>, SolveError> {
        self.taskmgr.solve_impl(data).await
    }

    /// [Doc](https://docs.capmonster.cloud/docs/captchas/geetest-task)
    pub async fn geetest_task_proxyless(
        &self,
        data: GeeTestTaskProxyless<'a>,
    ) -> Result<SvcResponse<GetTaskResultResp<GeeTestTaskProxylessResp>>, SolveError> {
        self.taskmgr.solve_impl(data).await
    }

    /// [Doc](https://docs.capmonster.cloud/docs/captchas/tunrstile-task)
    pub async fn turnstile_task(
        &self,
        data: CloudFlareChallengeWithProxy<'a>,
    ) -> Result<SvcResponse<GetTaskResultResp<TurnstileTaskResp>>, SolveError> {
        self.taskmgr.solve_impl(data).await
    }

    /// [Doc](https://docs.capmonster.cloud/docs/captchas/tunrstile-task)
    pub async fn turnstile_task_proxyless(
        &self,
        data: Turnstile<'a>,
    ) -> Result<SvcResponse<GetTaskResultResp<TurnstileTaskProxylessResp>>, SolveError> {
        self.taskmgr.solve_impl(data).await
    }

    /// [Doc](https://docs.capmonster.cloud/docs/captchas/recaptcha-click)
    pub async fn complex_image_task_recaptcha(
        &self,
        data: ComplexImageTaskData<'a>,
    ) -> Result<SvcResponse<GetTaskResultResp<TurnstileTaskProxylessResp>>, SolveError> {
        let cit = ComplexImageTask {
            class: "recaptcha",
            data,
        };

        self.taskmgr.solve_impl(cit).await
    }

    /// [Doc](https://docs.capmonster.cloud/docs/captchas/hcaptcha-click)
    pub async fn complex_image_task_hcaptcha(
        &self,
        data: ComplexImageTaskData<'a>,
    ) -> Result<SvcResponse<GetTaskResultResp<TurnstileTaskProxylessResp>>, SolveError> {
        let cit = ComplexImageTask {
            class: "hcaptcha",
            data,
        };

        self.taskmgr.solve_impl(cit).await
    }
    
    // /// [Doc](???)
    // pub async fn complex_image_task_funcaptcha(
    //     &self,
    //     data: ComplexImageTaskData<'a>,
    // ) -> Result<SvcResponse<GetTaskResultResp<TurnstileTaskProxylessResp>>, SolveError> {
    //     let data = ComplexImageTask {
    //         class: "funcaptcha",
    //         data,
    //     };
    //     
    //     self.taskmgr.solve_impl(data).await
    // }
    
    /// [Doc](https://docs.capmonster.cloud/docs/captchas/datadome)
    pub async fn custom_task_datadome(
        &self,
        data: CustomTaskData<'a>,
    ) -> Result<SvcResponse<GetTaskResultResp<CustomTaskResp>>, SolveError> {
        let ct = CustomTask {
            class: "DataDome",
            data,
        };
        
        self.taskmgr.solve_impl(ct).await
    }
    
    /// [Doc](https://docs.capmonster.cloud/docs/captchas/tendi)
    pub async fn custom_task_tendi(
        &self,
        data: CustomTaskData<'a>,
    ) -> Result<SvcResponse<GetTaskResultResp<CustomTaskResp>>, SolveError> {
        let ct = CustomTask {
            class: "TenDI",
            data,
        };
        
        self.taskmgr.solve_impl(ct).await
    }
    
    /// [Doc](https://docs.capmonster.cloud/docs/captchas/amazon-task)
    pub async fn amazon_task_proxyless(
        &self,
        data: AmazonTaskProxyless<'a>,
    ) -> Result<SvcResponse<GetTaskResultResp<AmazonTaskProxylessResp>>, SolveError> {
        self.taskmgr.solve_impl(data).await
    }
}
