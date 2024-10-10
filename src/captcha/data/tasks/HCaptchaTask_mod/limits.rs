use crate::cfg::limits::{request_interval_ms, result_timeout_ms, Limits, LimitsTrait};
use crate::{HCaptchaTaskProxyless};
use std::time::Duration;

// impl<'a> LimitsTrait for Limits<HCaptchaTask<'a>> {
//     const REQUEST_INTERVAL: Duration = Duration::from_millis(request_interval_ms::HCAPTCHA);
//     const RESULT_TIMEOUT: Duration = Duration::from_millis(result_timeout_ms::HCAPTCHA);
// }

impl<'a> LimitsTrait for Limits<HCaptchaTaskProxyless<'a>> {
    const REQUEST_INTERVAL: Duration = Duration::from_millis(request_interval_ms::HCAPTCHA);
    const RESULT_TIMEOUT: Duration = Duration::from_millis(result_timeout_ms::HCAPTCHA);
}
