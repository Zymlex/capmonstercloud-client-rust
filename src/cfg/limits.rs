use crate::*;
use std::marker::PhantomData;
use std::time::Duration;

pub(crate) const REQUEST_TIMEOUT_MS: u64 = 21_000;
pub(crate) const HTTP2_KEEP_ALIVE_INTERVAL_MS: u64 = 30_000;

pub(crate) const TASK_REQUEST_MAX: u8 = 120;

#[rustfmt::skip]
pub(crate) mod request_interval_ms {
    pub(crate) const IMAGE_TO_TEXT_TASK: u64 = 200;
    pub(crate) const RECAPTCHA:          u64 = 3_000;
    pub(crate) const FUNCAPTCHA:         u64 = 1_000;
    pub(crate) const HCAPTCHA:           u64 = 3_000;
    pub(crate) const GEETEST:            u64 = 1_000;
    pub(crate) const TURNSTILE:          u64 = 1_000;
    pub(crate) const COMPLEX_IMAGE:      u64 = IMAGE_TO_TEXT_TASK;
    pub(crate) const CUSTOM_TASK:        u64 = IMAGE_TO_TEXT_TASK;
    pub(crate) const AMAZON:             u64 = IMAGE_TO_TEXT_TASK;
}

#[rustfmt::skip]
pub(crate) mod result_timeout_ms {
    pub(crate) const IMAGE_TO_TEXT_TASK: u64 = 10_000;
    pub(crate) const RECAPTCHA:          u64 = 180_000;
    pub(crate) const FUNCAPTCHA:         u64 = 80_000;
    pub(crate) const HCAPTCHA:           u64 = 80_000;
    pub(crate) const GEETEST:            u64 = 80_000;
    pub(crate) const TURNSTILE:          u64 = 80_000;
    pub(crate) const COMPLEX_IMAGE:      u64 = IMAGE_TO_TEXT_TASK;
    pub(crate) const CUSTOM_TASK:        u64 = IMAGE_TO_TEXT_TASK;
    pub(crate) const AMAZON:             u64 = IMAGE_TO_TEXT_TASK;
}

pub(crate) struct Limits<T: TaskReqTrait> {
    #[allow(non_snake_case, reason = "hidden")]
    #[doc(hidden)]
    __: PhantomData<T>,
}

pub(crate) trait LimitsTrait {
    const REQUEST_INTERVAL: Duration;
    const RESULT_TIMEOUT: Duration;
}
