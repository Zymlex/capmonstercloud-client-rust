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
#![allow(
    clippy::wildcard_imports,
    clippy::module_name_repetitions,
    dead_code,
    clippy::redundant_pub_crate,
    clippy::missing_errors_doc,
    reason = "Dev"
)]

use error::GetUserAgentError;

pub use self::captcha::*;
// pub use self::utils::*;

use self::error::{
    CapMonsterCloudClientError::{self, ClientImpl},
    GetBalanceError, SolveError,
};
use self::taskmgr::Solver;

mod captcha;
mod cfg;
mod error;
mod taskmgr;
mod tests;
mod utils;

/// Call [`new`][CapMonsterCloudClient::new] to instantiate this struct.
pub struct CapMonsterCloudClient<'a> {
    taskmgr: Solver<'a>,
}

impl<'a> CapMonsterCloudClient<'a> {
    /// Creates new capmonster.cloud сlient
    pub fn new(client_key: &'a str) -> Result<Self, CapMonsterCloudClientError> {
        Ok(Self {
            taskmgr: Solver::new(client_key).map_err(ClientImpl)?,
        })
    }

    /// Get the amount of funds on the account.
    ///
    /// [Doc](https://docs.capmonster.cloud/docs/api/methods/get-balance)
    pub async fn get_user_agent_async(&self) -> Result<String, GetUserAgentError> {
        self.taskmgr.get_user_agent_async().await
    }

    /// Get the amount of funds on the account.
    ///
    /// [Doc](https://docs.capmonster.cloud/docs/api/methods/get-balance)
    pub async fn get_balance_async(&self) -> Result<f64, GetBalanceError> {
        self.taskmgr.get_balance_async().await
    }

    /// [Doc](https://docs.capmonster.cloud/docs/captchas/image-to-text)
    pub async fn image_to_text(&self, data: ImageToTextTask<'a>) -> Result<String, SolveError> {
        self.taskmgr
            .solve_impl::<_, ImageToTextTaskResp>(data)
            .await
            .map(|res| res.text)
    }

    /// [Doc](https://docs.capmonster.cloud/docs/captchas/no-captcha-task)
    pub async fn recaptcha_v2(
        &self,
        data: RecaptchaV2TaskProxyless<'a>,
    ) -> Result<String, SolveError> {
        self.taskmgr
            .solve_impl::<_, RecaptchaV2TaskProxylessResp>(data)
            .await
            .map(|res| res.gRecaptchaResponse)
    }

    // /// [Doc](https://docs.capmonster.cloud/docs/captchas/no-captcha-task)
    // pub async fn recaptcha_v2_proxy(
    //     &self,
    //     data: RecaptchaV2Task<'a>,
    // ) -> Result<SvcResponse<GetTaskResultResp<RecaptchaV2TaskResp>>, SolveError> {
    //     self.taskmgr.solve_impl(data).await
    // }

    /// [Doc](https://docs.capmonster.cloud/docs/captchas/recaptcha-v3-task)
    pub async fn recaptcha_v3(
        &self,
        data: RecaptchaV3TaskProxyless<'a>,
    ) -> Result<String, SolveError> {
        self.taskmgr
            .solve_impl::<_, RecaptchaV3TaskProxylessResp>(data)
            .await
            .map(|res| res.gRecaptchaResponse)
    }

    /// [Doc](https://docs.capmonster.cloud/docs/captchas/recaptcha-v2-enterprise-task)
    pub async fn recaptcha_v2_enterprise(
        &self,
        data: RecaptchaV2EnterpriseTaskProxyless<'a>,
    ) -> Result<String, SolveError> {
        self.taskmgr
            .solve_impl::<_, RecaptchaV2EnterpriseTaskProxylessResp>(data)
            .await
            .map(|res| res.gRecaptchaResponse)
    }

    // /// [Doc](https://docs.capmonster.cloud/docs/captchas/recaptcha-click)
    // pub async fn recaptcha_complex(
    //     &self,
    //     data: ComplexImageTaskData<'a>,
    // ) -> Result<<GetTaskResultResp<> as SvcRespTypeTrait>::Value, SolveError> {
    //     let cit = ComplexImageTask {
    //         class: "recaptcha",
    //         data,
    //     };
    //     self.taskmgr.solve_impl(cit).await
    // }

    // /// [Doc](https://docs.capmonster.cloud/docs/captchas/recaptcha-v2-enterprise-task)
    // pub async fn recaptcha_v2_enterprise_proxy(
    //     &self,
    //     data: RecaptchaV2EnterpriseTask<'a>,
    // ) -> Result<SvcResponse<GetTaskResultResp<RecaptchaV2EnterpriseTaskResp>>, SolveError> {
    //     self.taskmgr.solve_impl(data).await
    // }

    // /// [Doc](???)
    // pub async fn funcaptcha_proxy(
    //     &self,
    //     data: FunCaptchaTask<'a>,
    // ) -> Result<SvcResponse<GetTaskResultResp<FunCaptchaTaskResp>>, SolveError> {
    //     self.taskmgr.solve_impl(data).await
    // }
    //
    // /// [Doc](???)
    // pub async fn funcaptcha(
    //     &self,
    //     data: FunCaptchaTaskProxyless<'a>,
    // ) -> Result<SvcResponse<GetTaskResultResp<FunCaptchaTaskProxylessResp>>, SolveError> {
    //     self.taskmgr.solve_impl(data).await
    // }

    // /// [Doc](???)
    // pub async fn funcaptcha_complex(
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

    // /// [Doc](https://docs.capmonster.cloud/docs/captchas/hcaptcha-task)
    // pub async fn hcaptcha_proxy(
    //     &self,
    //     data: HCaptchaTask<'a>,
    // ) -> Result<SvcResponse<GetTaskResultResp<HCaptchaTaskResp>>, SolveError> {
    //     self.taskmgr.solve_impl(data).await
    // }

    /// [Doc](https://docs.capmonster.cloud/docs/captchas/hcaptcha-task)
    // pub async fn hcaptcha(&self, data: HCaptchaTaskProxyless<'a>) -> Result<String, SolveError> {
    //     self.taskmgr
    //         .solve_impl::<_, HCaptchaTaskProxylessResp>(data)
    //         .await
    //         .map(|res| res.gRecaptchaResponse)
    // }

    // /// [Doc](https://docs.capmonster.cloud/docs/captchas/hcaptcha-click)
    // pub async fn hcaptcha_complex(
    //     &self,
    //     data: ComplexImageTaskData<'a>,
    // ) -> Result<<GetTaskResultResp<> as SvcRespTypeTrait>::Value, SolveError> {
    //     let cit = ComplexImageTask {
    //         class: "hcaptcha",
    //         data,
    //     };

    // /// [Doc](https://docs.capmonster.cloud/docs/captchas/geetest-task)
    // pub async fn geetest_proxy(
    //     &self,
    //     data: GeeTestTask<'a>,
    // ) -> Result<SvcResponse<GetTaskResultResp<GeeTestTaskResp>>, SolveError> {
    //     self.taskmgr.solve_impl(data).await
    // }

    /// [Doc](https://docs.capmonster.cloud/docs/captchas/geetest-task)
    pub async fn geetest(
        &self,
        data: GeeTestTaskProxyless<'a>,
    ) -> Result<GeeTestTaskProxylessResp, SolveError> {
        self.taskmgr
            .solve_impl::<_, GeeTestTaskProxylessResp>(data)
            .await
    }

    /// [Doc](https://docs.capmonster.cloud/docs/captchas/tunrstile-task)
    pub async fn turnstile(&self, data: Turnstile<'a>) -> Result<TurnstileTaskResp, SolveError> {
        let tt = TurnstileTask {
            r#type: "TurnstileTaskProxyless",
            cloudflareTaskType: None,
            data: TTData::Turnstile(data),
        };

        self.taskmgr.solve_impl::<_, TurnstileTaskResp>(tt).await
    }

    /// [Doc](https://docs.capmonster.cloud/docs/captchas/tunrstile-task)
    pub async fn cloudflare_token(
        &self,
        data: CloudFlareToken<'a>,
    ) -> Result<TurnstileTaskResp, SolveError> {
        let tt = TurnstileTask {
            r#type: "TurnstileTaskProxyless",
            cloudflareTaskType: Some("token"),
            data: TTData::CloudFlareToken(data),
        };

        self.taskmgr.solve_impl::<_, TurnstileTaskResp>(tt).await
    }

    /// [Doc](https://docs.capmonster.cloud/docs/captchas/tunrstile-task)
    pub async fn cloudflare_cf_clearance(
        &self,
        data: CloudFlareCFClearance<'a>,
    ) -> Result<TurnstileTaskResp, SolveError> {
        let tt = TurnstileTask {
            r#type: "TurnstileTask",
            cloudflareTaskType: Some("cf_clearance"),
            data: TTData::CloudFlareCFClearance(data),
        };

        self.taskmgr.solve_impl::<_, TurnstileTaskResp>(tt).await
    }

    /// [Doc](https://docs.capmonster.cloud/docs/captchas/datadome)
    pub async fn datadome(
        &self,
        data: CustomTaskData<'a>,
    ) -> Result<CustomTaskDomains, SolveError> {
        let ct = CustomTask {
            class: "DataDome",
            data,
        };

        self.taskmgr
            .solve_impl::<_, CustomTaskResp>(ct)
            .await
            .map(|res| res.domains)
    }

    /// [Doc](https://docs.capmonster.cloud/docs/captchas/tendi)
    pub async fn tendi(&self, data: CustomTaskData<'a>) -> Result<CustomTaskDomains, SolveError> {
        let ct = CustomTask {
            class: "TenDI",
            data,
        };

        self.taskmgr
            .solve_impl::<_, CustomTaskResp>(ct)
            .await
            .map(|res| res.domains)
    }

    /// [Doc](https://docs.capmonster.cloud/docs/captchas/amazon-task)
    pub async fn amazon(
        &self,
        data: AmazonTaskProxyless<'a>,
    ) -> Result<AmazonTaskProxylessResp, SolveError> {
        self.taskmgr
            .solve_impl::<_, AmazonTaskProxylessResp>(data)
            .await
    }
}
