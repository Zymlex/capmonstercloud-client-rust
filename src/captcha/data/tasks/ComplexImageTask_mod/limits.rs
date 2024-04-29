use crate::captcha::ComplexImageTask;
use crate::cfg::limits::{request_interval_ms, result_timeout_ms, Limits, LimitsTrait};
use std::time::Duration;

impl<'a> LimitsTrait for Limits<ComplexImageTask<'a>> {
    const REQUEST_INTERVAL: Duration = Duration::from_millis(request_interval_ms::COMPLEX_IMAGE);
    const RESULT_TIMEOUT: Duration = Duration::from_millis(result_timeout_ms::COMPLEX_IMAGE);
}
