use crate::captcha::CustomTask;
use crate::cfg::limits::{request_interval_ms, result_timeout_ms, Limits, LimitsTrait};
use std::time::Duration;

impl<'a> LimitsTrait for Limits<CustomTask<'a>> {
    const REQUEST_INTERVAL: Duration = Duration::from_millis(request_interval_ms::CUSTOM_TASK);
    const RESULT_TIMEOUT: Duration = Duration::from_millis(result_timeout_ms::CUSTOM_TASK);
}
