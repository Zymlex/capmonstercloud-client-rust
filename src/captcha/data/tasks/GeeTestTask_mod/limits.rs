use crate::cfg::limits::{request_interval_ms, result_timeout_ms, Limits, LimitsTrait};
use crate::{GeeTestTask, GeeTestTaskProxyless};
use std::time::Duration;

impl<'a> LimitsTrait for Limits<GeeTestTask<'a>> {
    const REQUEST_INTERVAL: Duration = Duration::from_millis(request_interval_ms::GEETEST);
    const RESULT_TIMEOUT: Duration = Duration::from_millis(result_timeout_ms::GEETEST);
}

impl<'a> LimitsTrait for Limits<GeeTestTaskProxyless<'a>> {
    const REQUEST_INTERVAL: Duration = Duration::from_millis(request_interval_ms::GEETEST);
    const RESULT_TIMEOUT: Duration = Duration::from_millis(result_timeout_ms::GEETEST);
}
