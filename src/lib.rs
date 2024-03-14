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

pub use self::captcha::*;

use self::cfg::Config;
use self::error::CapMonsterCloudClientError::ClientImplError;
use self::error::{CapMonsterCloudClientError, GetBalanceError, SolveError};
use self::taskmgr::Solver;

mod captcha;
mod cfg;
mod error;
mod taskmgr;
mod tests;

pub struct CapMonsterCloudClient<'a> {
    taskmgr: Solver<'a>,
}

impl<'a> CapMonsterCloudClient<'a> {
    /// Creates new capmonster.cloud сlient
    pub fn new(client_key: &'a str) -> Result<Self, CapMonsterCloudClientError>
    {
        Self::new_ex(Config::new(client_key)?)
    }

    /// Creates new capmonster.cloud сlient with additional options
    fn new_ex(cfg: Config<'a>) -> Result<Self, CapMonsterCloudClientError> {
        Ok(Self {
            taskmgr: Solver::new(cfg).map_err(ClientImplError)?,
        })
    }

    /// Get the amount of funds on the account.
    ///
    /// https://zennolab.atlassian.net/wiki/x/SAAK
    pub async fn get_balance_async(&self) -> Result<SvcResponse<GetBalanceResp>, GetBalanceError> {
        self.taskmgr.get_balance_async().await
    }

    /// https://zennolab.atlassian.net/wiki/x/bQAK
    pub async fn image_to_text_task(
        &self,
        data: ImageToTextTask<'a>,
    ) -> Result<SvcResponse<GetTaskResultResp<ImageToTextTaskResp>>, SolveError> {
        self.taskmgr.solve_impl(data).await
    }

    /// https://zennolab.atlassian.net/wiki/x/AQA_Fg
    pub async fn no_captcha_task_proxyless(
        &self,
        data: NoCaptchaTaskProxyless<'a>,
    ) -> Result<SvcResponse<GetTaskResultResp<NoCaptchaTaskProxylessResp>>, SolveError> {
        self.taskmgr.solve_impl(data).await
    }

    /// https://zennolab.atlassian.net/wiki/x/FYCSK
    pub async fn no_captcha_task(
        &self,
        data: NoCaptchaTask<'a>,
    ) -> Result<SvcResponse<GetTaskResultResp<NoCaptchaTaskResp>>, SolveError> {
        self.taskmgr.solve_impl(data).await
    }

    /// https://zennolab.atlassian.net/wiki/x/AQA_Fg
    pub async fn recaptcha_v2_task_proxyless(
        &self,
        data: NoCaptchaTaskProxyless<'a>,
    ) -> Result<SvcResponse<GetTaskResultResp<NoCaptchaTaskProxylessResp>>, SolveError> {
        self.no_captcha_task_proxyless(data).await
    }

    /// https://zennolab.atlassian.net/wiki/x/FYCSK
    pub async fn recaptcha_v2_task(
        &self,
        data: NoCaptchaTask<'a>,
    ) -> Result<SvcResponse<GetTaskResultResp<NoCaptchaTaskResp>>, SolveError> {
        self.no_captcha_task(data).await
    }

    /// https://zennolab.atlassian.net/wiki/x/EoDJIQ
    pub async fn recaptcha_v3_task_proxyless(
        &self,
        data: RecaptchaV3TaskProxyless<'a>,
    ) -> Result<SvcResponse<GetTaskResultResp<RecaptchaV3TaskProxylessResp>>, SolveError> {
        self.taskmgr.solve_impl(data).await
    }

    /// https://zennolab.atlassian.net/wiki/x/FYDXgQ
    pub async fn recaptcha_v2_enterprise_task_proxyless(
        &self,
        data: RecaptchaV2EnterpriseTaskProxyless<'a>,
    ) -> Result<SvcResponse<GetTaskResultResp<RecaptchaV2EnterpriseTaskProxylessResp>>, SolveError>
    {
        self.taskmgr.solve_impl(data).await
    }

    /// https://zennolab.atlassian.net/wiki/x/AYDigQ
    pub async fn recaptcha_v2_enterprise_task(
        &self,
        data: RecaptchaV2EnterpriseTask<'a>,
    ) -> Result<SvcResponse<GetTaskResultResp<RecaptchaV2EnterpriseTaskResp>>, SolveError> {
        self.taskmgr.solve_impl(data).await
    }

    /// https://zennolab.atlassian.net/wiki/x/OYDbKw
    pub async fn funcaptcha_task(
        &self,
        data: FunCaptchaTask<'a>,
    ) -> Result<SvcResponse<GetTaskResultResp<FunCaptchaTaskResp>>, SolveError> {
        self.taskmgr.solve_impl(data).await
    }

    /// https://zennolab.atlassian.net/wiki/x/FwBdJg
    pub async fn funcaptcha_task_proxyless(
        &self,
        data: FunCaptchaTaskProxyless<'a>,
    ) -> Result<SvcResponse<GetTaskResultResp<FunCaptchaTaskProxylessResp>>, SolveError> {
        self.taskmgr.solve_impl(data).await
    }

    /// https://zennolab.atlassian.net/wiki/x/HAC4Rw
    pub async fn hcaptcha_task(
        &self,
        data: HCaptchaTask<'a>,
    ) -> Result<SvcResponse<GetTaskResultResp<HCaptchaTaskResp>>, SolveError> {
        self.taskmgr.solve_impl(data).await
    }

    /// https://zennolab.atlassian.net/wiki/x/EQC4Rw
    pub async fn hcaptcha_task_proxyless(
        &self,
        data: HCaptchaTaskProxyless<'a>,
    ) -> Result<SvcResponse<GetTaskResultResp<HCaptchaTaskProxylessResp>>, SolveError> {
        self.taskmgr.solve_impl(data).await
    }

    /// https://zennolab.atlassian.net/wiki/x/J4Cncw
    pub async fn geetest_task(
        &self,
        data: GeeTestTask<'a>,
    ) -> Result<SvcResponse<GetTaskResultResp<GeeTestTaskResp>>, SolveError> {
        self.taskmgr.solve_impl(data).await
    }

    /// https://zennolab.atlassian.net/wiki/x/KoCmcw
    pub async fn geetest_task_proxyless(
        &self,
        data: GeeTestTaskProxyless<'a>,
    ) -> Result<SvcResponse<GetTaskResultResp<GeeTestTaskProxylessResp>>, SolveError> {
        self.taskmgr.solve_impl(data).await
    }

    /// https://zennolab.atlassian.net/wiki/x/CoCShg
    pub async fn turnstile_task(
        &self,
        data: TurnstileTask<'a>,
    ) -> Result<SvcResponse<GetTaskResultResp<TurnstileTaskResp>>, SolveError> {
        self.taskmgr.solve_impl(data).await
    }

    /// https://zennolab.atlassian.net/wiki/x/B4CUhg
    pub async fn turnstile_task_proxyless(
        &self,
        data: TurnstileTaskProxyless<'a>,
    ) -> Result<SvcResponse<GetTaskResultResp<TurnstileTaskProxylessResp>>, SolveError> {
        self.taskmgr.solve_impl(data).await
    }

    /// https://zennolab.atlassian.net/wiki/x/AQDghw
    pub async fn complex_image_task_recaptcha(
        &self,
        data: ComplexImageTaskData<'a>,
    ) -> Result<SvcResponse<GetTaskResultResp<TurnstileTaskProxylessResp>>, SolveError> {
        let data = ComplexImageTask {
            class: "recaptcha",
            data,
        };

        self.taskmgr.solve_impl(data).await
    }

    /// https://zennolab.atlassian.net/wiki/x/IgDfhw
    pub async fn complex_image_task_hcaptcha(
        &self,
        data: ComplexImageTaskData<'a>,
    ) -> Result<SvcResponse<GetTaskResultResp<TurnstileTaskProxylessResp>>, SolveError> {
        let data = ComplexImageTask {
            class: "hcaptcha",
            data,
        };

        self.taskmgr.solve_impl(data).await
    }

    /// https://zennolab.atlassian.net/wiki/x/AQC0i
    pub async fn complex_image_task_funcaptcha(
        &self,
        data: ComplexImageTaskData<'a>,
    ) -> Result<SvcResponse<GetTaskResultResp<TurnstileTaskProxylessResp>>, SolveError> {
        let data = ComplexImageTask {
            class: "funcaptcha",
            data,
        };

        self.taskmgr.solve_impl(data).await
    }
}
