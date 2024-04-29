use std::time::Duration;
use crate::AmazonTaskProxyless;
use crate::cfg::limits::{Limits, LimitsTrait, request_interval_ms, result_timeout_ms};

impl<'a> LimitsTrait for Limits<AmazonTaskProxyless<'a>> {
    const REQUEST_INTERVAL: Duration = Duration::from_millis(request_interval_ms::AMAZON);
    const RESULT_TIMEOUT: Duration = Duration::from_millis(result_timeout_ms::AMAZON);
}