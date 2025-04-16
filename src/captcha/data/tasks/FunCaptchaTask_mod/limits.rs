use crate::FunCaptchaTaskProxyless;
use crate::cfg::limits::{Limits, LimitsTrait, request_interval_ms, result_timeout_ms};
use std::time::Duration;

// impl<'a> LimitsTrait for Limits<FunCaptchaTask<'a>> {
//     const REQUEST_INTERVAL: Duration = Duration::from_millis(request_interval_ms::FUNCAPTCHA);
//     const RESULT_TIMEOUT: Duration = Duration::from_millis(result_timeout_ms::FUNCAPTCHA);
// }

impl<'a> LimitsTrait for Limits<FunCaptchaTaskProxyless<'a>> {
    const REQUEST_INTERVAL: Duration = Duration::from_millis(request_interval_ms::FUNCAPTCHA);
    const RESULT_TIMEOUT: Duration = Duration::from_millis(result_timeout_ms::FUNCAPTCHA);
}
