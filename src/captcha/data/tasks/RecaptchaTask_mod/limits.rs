use crate::cfg::limits::{request_interval_ms, result_timeout_ms, Limits, LimitsTrait};
use crate::{
    RecaptchaV2EnterpriseTask, RecaptchaV2EnterpriseTaskProxyless, RecaptchaV2Task,
    RecaptchaV2TaskProxyless, RecaptchaV3TaskProxyless,
};
use std::time::Duration;

impl<'a> LimitsTrait for Limits<RecaptchaV2TaskProxyless<'a>> {
    const REQUEST_INTERVAL: Duration = Duration::from_millis(request_interval_ms::RECAPTCHA);
    const RESULT_TIMEOUT: Duration = Duration::from_millis(result_timeout_ms::RECAPTCHA);
}

impl<'a> LimitsTrait for Limits<RecaptchaV2Task<'a>> {
    const REQUEST_INTERVAL: Duration = Duration::from_millis(request_interval_ms::RECAPTCHA);
    const RESULT_TIMEOUT: Duration = Duration::from_millis(result_timeout_ms::RECAPTCHA);
}

impl<'a> LimitsTrait for Limits<RecaptchaV3TaskProxyless<'a>> {
    const REQUEST_INTERVAL: Duration = Duration::from_millis(request_interval_ms::RECAPTCHA);
    const RESULT_TIMEOUT: Duration = Duration::from_millis(result_timeout_ms::RECAPTCHA);
}

impl<'a> LimitsTrait for Limits<RecaptchaV2EnterpriseTask<'a>> {
    const REQUEST_INTERVAL: Duration = Duration::from_millis(request_interval_ms::RECAPTCHA);
    const RESULT_TIMEOUT: Duration = Duration::from_millis(result_timeout_ms::RECAPTCHA);
}

impl<'a> LimitsTrait for Limits<RecaptchaV2EnterpriseTaskProxyless<'a>> {
    const REQUEST_INTERVAL: Duration = Duration::from_millis(request_interval_ms::RECAPTCHA);
    const RESULT_TIMEOUT: Duration = Duration::from_millis(result_timeout_ms::RECAPTCHA);
}
