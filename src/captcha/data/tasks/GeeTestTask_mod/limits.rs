use crate::GeeTestTaskProxyless;
use crate::cfg::limits::{Limits, LimitsTrait, request_interval_ms, result_timeout_ms};
use std::time::Duration;

// impl<'a> LimitsTrait for Limits<GeeTestTask<'a>> {
//     const REQUEST_INTERVAL: Duration = Duration::from_millis(request_interval_ms::GEETEST);
//     const RESULT_TIMEOUT: Duration = Duration::from_millis(result_timeout_ms::GEETEST);
// }

impl<'a> LimitsTrait for Limits<GeeTestTaskProxyless<'a>> {
    const REQUEST_INTERVAL: Duration = Duration::from_millis(request_interval_ms::GEETEST);
    const RESULT_TIMEOUT: Duration = Duration::from_millis(result_timeout_ms::GEETEST);
}
