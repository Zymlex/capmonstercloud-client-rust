use crate::ImageToTextTask;
use crate::cfg::limits::{Limits, LimitsTrait};
use std::time::Duration;

impl<'a> LimitsTrait for Limits<ImageToTextTask<'a>> {
    const REQUEST_INTERVAL: Duration =
        Duration::from_millis(crate::cfg::limits::request_interval_ms::IMAGE_TO_TEXT_TASK);
    const RESULT_TIMEOUT: Duration =
        Duration::from_millis(crate::cfg::limits::result_timeout_ms::IMAGE_TO_TEXT_TASK);
}
