use crate::cfg::limits::{request_interval_ms, result_timeout_ms, Limits, LimitsTrait};
use crate::{CloudFlareChallengeWithProxy, Turnstile};
use std::time::Duration;

impl<'a> LimitsTrait for Limits<CloudFlareChallengeWithProxy<'a>> {
    const REQUEST_INTERVAL: Duration = Duration::from_millis(request_interval_ms::TURNSTILE);
    const RESULT_TIMEOUT: Duration = Duration::from_millis(result_timeout_ms::TURNSTILE);
}

impl<'a> LimitsTrait for Limits<Turnstile<'a>> {
    const REQUEST_INTERVAL: Duration = Duration::from_millis(request_interval_ms::TURNSTILE);
    const RESULT_TIMEOUT: Duration = Duration::from_millis(result_timeout_ms::TURNSTILE);
}
