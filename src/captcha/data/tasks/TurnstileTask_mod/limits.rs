use crate::TurnstileTask;
use crate::cfg::limits::{Limits, LimitsTrait, request_interval_ms, result_timeout_ms};
use std::time::Duration;

impl<'a> LimitsTrait for Limits<TurnstileTask<'a>> {
    const REQUEST_INTERVAL: Duration = Duration::from_millis(request_interval_ms::TURNSTILE);
    const RESULT_TIMEOUT: Duration = Duration::from_millis(result_timeout_ms::TURNSTILE);
}
