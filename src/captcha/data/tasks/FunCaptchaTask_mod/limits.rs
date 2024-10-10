use crate::cfg::limits::{request_interval_ms, result_timeout_ms, Limits, LimitsTrait};
use crate::{FunCaptchaTaskProxyless};
use std::time::Duration;

// impl<'a> LimitsTrait for Limits<FunCaptchaTask<'a>> {
//     const REQUEST_INTERVAL: Duration = Duration::from_millis(request_interval_ms::FUNCAPTCHA);
//     const RESULT_TIMEOUT: Duration = Duration::from_millis(result_timeout_ms::FUNCAPTCHA);
// }

impl<'a> LimitsTrait for Limits<FunCaptchaTaskProxyless<'a>> {
    const REQUEST_INTERVAL: Duration = Duration::from_millis(request_interval_ms::FUNCAPTCHA);
    const RESULT_TIMEOUT: Duration = Duration::from_millis(result_timeout_ms::FUNCAPTCHA);
}
