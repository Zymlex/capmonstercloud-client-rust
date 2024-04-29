use crate::{
    ComplexImageTask, FunCaptchaTask, FunCaptchaTaskProxyless, GeeTestTask, GeeTestTaskProxyless,
    HCaptchaTask, HCaptchaTaskProxyless, ImageToTextTask, NoCaptchaTask, NoCaptchaTaskProxyless,
    RecaptchaV2EnterpriseTask, RecaptchaV2EnterpriseTaskProxyless, RecaptchaV3TaskProxyless,
    TaskReqTrait, TurnstileTask, TurnstileTaskProxyless,
};
use std::marker::PhantomData;
use std::time::Duration;

pub(crate) const REQUEST_TIMEOUT_MS: u64 = 21_000;
pub(crate) const HTTP2_KEEP_ALIVE_INTERVAL_MS: u64 = 30_000;

pub(crate) const TASK_REQUEST_MAX: u8 = 120;

mod request_interval_ms {
    pub(crate) const ITT: u64 = 200; // ImageToTextTask
    pub(crate) const RC:  u64 = 3_000;
    pub(crate) const FC:  u64 = 1_000;
    pub(crate) const HC:  u64 = 3_000;
    pub(crate) const GT:  u64 = 1_000;
    pub(crate) const T:   u64 = 1_000;
    pub(crate) const CI:  u64 = ITT;
}

mod result_timeout_ms {
    pub(crate) const ITT: u64 = 10_000; // ImageToTextTask
    pub(crate) const RC:  u64 = 180_000;
    pub(crate) const FC:  u64 = 80_000;
    pub(crate) const HC:  u64 = 80_000;
    pub(crate) const GT:  u64 = 80_000;
    pub(crate) const T:   u64 = 80_000;
    pub(crate) const CI:  u64 = ITT;
}

pub(crate) struct Limits<T: TaskReqTrait> {
    #[allow(non_snake_case)]
    #[doc(hidden)]
    __: PhantomData<T>,
}

pub(crate) trait LimitsTrait {
    const REQUEST_INTERVAL: Duration;
    const RESULT_TIMEOUT: Duration;
}

//TODO Macros for all this:

impl<'a> LimitsTrait for Limits<ImageToTextTask<'a>> {
    const RESULT_TIMEOUT: Duration = Duration::from_millis(result_timeout_ms::ITT);
}

impl<'a> LimitsTrait for Limits<NoCaptchaTaskProxyless<'a>> {
    const REQUEST_INTERVAL: Duration = Duration::from_millis(request_interval_ms::RC);
    const RESULT_TIMEOUT: Duration = Duration::from_millis(result_timeout_ms::RC);
}

impl<'a> LimitsTrait for Limits<NoCaptchaTask<'a>> {
    const REQUEST_INTERVAL: Duration = Duration::from_millis(request_interval_ms::RC);
    const RESULT_TIMEOUT: Duration = Duration::from_millis(result_timeout_ms::RC);
}

impl<'a> LimitsTrait for Limits<RecaptchaV3TaskProxyless<'a>> {
    const REQUEST_INTERVAL: Duration = Duration::from_millis(request_interval_ms::RC);
    const RESULT_TIMEOUT: Duration = Duration::from_millis(result_timeout_ms::RC);
}

impl<'a> LimitsTrait for Limits<RecaptchaV2EnterpriseTask<'a>> {
    const REQUEST_INTERVAL: Duration = Duration::from_millis(request_interval_ms::RC);
    const RESULT_TIMEOUT: Duration = Duration::from_millis(result_timeout_ms::RC);
}

impl<'a> LimitsTrait for Limits<RecaptchaV2EnterpriseTaskProxyless<'a>> {
    const REQUEST_INTERVAL: Duration = Duration::from_millis(request_interval_ms::RC);
    const RESULT_TIMEOUT: Duration = Duration::from_millis(result_timeout_ms::RC);
}

impl<'a> LimitsTrait for Limits<FunCaptchaTask<'a>> {
    const REQUEST_INTERVAL: Duration = Duration::from_millis(request_interval_ms::FC);
    const RESULT_TIMEOUT: Duration = Duration::from_millis(result_timeout_ms::FC);
}

impl<'a> LimitsTrait for Limits<FunCaptchaTaskProxyless<'a>> {
    const REQUEST_INTERVAL: Duration = Duration::from_millis(request_interval_ms::FC);
    const RESULT_TIMEOUT: Duration = Duration::from_millis(result_timeout_ms::FC);
}

impl<'a> LimitsTrait for Limits<HCaptchaTask<'a>> {
    const REQUEST_INTERVAL: Duration = Duration::from_millis(request_interval_ms::HC);
    const RESULT_TIMEOUT: Duration = Duration::from_millis(result_timeout_ms::HC);
}

impl<'a> LimitsTrait for Limits<HCaptchaTaskProxyless<'a>> {
    const REQUEST_INTERVAL: Duration = Duration::from_millis(request_interval_ms::HC);
    const RESULT_TIMEOUT: Duration = Duration::from_millis(result_timeout_ms::HC);
}

impl<'a> LimitsTrait for Limits<GeeTestTask<'a>> {
    const REQUEST_INTERVAL: Duration = Duration::from_millis(request_interval_ms::GT);
    const RESULT_TIMEOUT: Duration = Duration::from_millis(result_timeout_ms::GT);
}

impl<'a> LimitsTrait for Limits<GeeTestTaskProxyless<'a>> {
    const REQUEST_INTERVAL: Duration = Duration::from_millis(request_interval_ms::GT);
    const RESULT_TIMEOUT: Duration = Duration::from_millis(result_timeout_ms::GT);
}

impl<'a> LimitsTrait for Limits<TurnstileTask<'a>> {
    const REQUEST_INTERVAL: Duration = Duration::from_millis(request_interval_ms::T);
    const RESULT_TIMEOUT: Duration = Duration::from_millis(result_timeout_ms::T);
}

impl<'a> LimitsTrait for Limits<TurnstileTaskProxyless<'a>> {
    const REQUEST_INTERVAL: Duration = Duration::from_millis(request_interval_ms::T);
    const RESULT_TIMEOUT: Duration = Duration::from_millis(result_timeout_ms::T);
}

impl<'a> LimitsTrait for Limits<ComplexImageTask<'a>> {
    const REQUEST_INTERVAL: Duration = Duration::from_millis(request_interval_ms::CI);
    const RESULT_TIMEOUT: Duration = Duration::from_millis(result_timeout_ms::CI);
}
