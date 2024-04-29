#![allow(non_upper_case_globals)]

use crate::requests::tasks_data::*;
use crate::requests::TaskReqTrait;
use std::marker::PhantomData;
use std::time::{Duration, Instant};

pub(crate) const REQUEST_TIMEOUT: Duration = Duration::from_secs(21);

// request intervals in ms
const ITT_REQUEST_INTERVAL: Duration = Duration::from_millis(200); // for ImageToTextTask
const RC_REQUEST_INTERVAL: Duration = Duration::from_secs(3);
const FC_REQUEST_INTERVAL: Duration = Duration::from_secs(1);
const HC_REQUEST_INTERVAL: Duration = Duration::from_secs(3);
const GT_REQUEST_INTERVAL: Duration = Duration::from_secs(1);

// result timeouts
const ITT_RESULT_TIMEOUT: Duration = Duration::from_secs(10);
const RC_RESULT_TIMEOUT: Duration = Duration::from_secs(180);
const FC_RESULT_TIMEOUT: Duration = Duration::from_secs(80);
const HC_RESULT_TIMEOUT: Duration = Duration::from_secs(80);
const GT_RESULT_TIMEOUT: Duration = Duration::from_secs(80);

pub(crate) struct Limits<T: TaskReqTrait> {
    request_interval: Duration,
    result_timeout: Duration,
    #[allow(non_snake_case)]
    __: PhantomData<T>,
}

impl<T: TaskReqTrait> Limits<T> {
    pub(crate) fn request_interval(&self) -> Duration {
        self.request_interval
    }
    pub(crate) fn result_timeout(&self) -> Duration {
        self.result_timeout
    }
}

pub(crate) trait LimitsTrait {
    fn new() -> Self;
}

impl<'a> LimitsTrait for Limits<ImageToTextTaskReq<'a>> {
    fn new() -> Self {
        Self {
            request_interval: ITT_REQUEST_INTERVAL,
            result_timeout: ITT_RESULT_TIMEOUT,
            __: PhantomData,
        }
    }
}

impl<'a> LimitsTrait for Limits<NoCaptchaTaskProxylessReq<'a>> {
    fn new() -> Self {
        Self {
            request_interval: RC_REQUEST_INTERVAL,
            result_timeout: RC_RESULT_TIMEOUT,
            __: PhantomData,
        }
    }
}

impl<'a> LimitsTrait for Limits<NoCaptchaTaskReq<'a>> {
    fn new() -> Self {
        Self {
            request_interval: RC_REQUEST_INTERVAL,
            result_timeout: RC_RESULT_TIMEOUT,
            __: PhantomData,
        }
    }
}

impl<'a> LimitsTrait for Limits<RecaptchaV3TaskProxylessReq<'a>> {
    fn new() -> Self {
        Self {
            request_interval: RC_REQUEST_INTERVAL,
            result_timeout: RC_RESULT_TIMEOUT,
            __: PhantomData,
        }
    }
}

impl<'a> LimitsTrait for Limits<RecaptchaV2EnterpriseTaskReq<'a>> {
    fn new() -> Self {
        Self {
            request_interval: RC_REQUEST_INTERVAL,
            result_timeout: RC_RESULT_TIMEOUT,
            __: PhantomData,
        }
    }
}

impl<'a> LimitsTrait for Limits<RecaptchaV2EnterpriseTaskProxylessReq<'a>> {
    fn new() -> Self {
        Self {
            request_interval: RC_REQUEST_INTERVAL,
            result_timeout: RC_RESULT_TIMEOUT,
            __: PhantomData,
        }
    }
}

impl<'a> LimitsTrait for Limits<FunCaptchaTaskReq<'a>> {
    fn new() -> Self {
        Self {
            request_interval: FC_REQUEST_INTERVAL,
            result_timeout: FC_RESULT_TIMEOUT,
            __: PhantomData,
        }
    }
}

impl<'a> LimitsTrait for Limits<FunCaptchaTaskProxylessReq<'a>> {
    fn new() -> Self {
        Self {
            request_interval: FC_REQUEST_INTERVAL,
            result_timeout: FC_RESULT_TIMEOUT,
            __: PhantomData,
        }
    }
}

impl<'a> LimitsTrait for Limits<HCaptchaTaskReq<'a>> {
    fn new() -> Self {
        Self {
            request_interval: HC_REQUEST_INTERVAL,
            result_timeout: HC_RESULT_TIMEOUT,
            __: PhantomData,
        }
    }
}

impl<'a> LimitsTrait for Limits<HCaptchaTaskProxylessReq<'a>> {
    fn new() -> Self {
        Self {
            request_interval: HC_REQUEST_INTERVAL,
            result_timeout: HC_RESULT_TIMEOUT,
            __: PhantomData,
        }
    }
}

impl<'a> LimitsTrait for Limits<GeeTestTaskReq<'a>> {
    fn new() -> Self {
        Self {
            request_interval: GT_REQUEST_INTERVAL,
            result_timeout: GT_RESULT_TIMEOUT,
            __: PhantomData,
        }
    }
}

impl<'a> LimitsTrait for Limits<GeeTestTaskProxylessReq<'a>> {
    fn new() -> Self {
        Self {
            request_interval: GT_REQUEST_INTERVAL,
            result_timeout: GT_RESULT_TIMEOUT,
            __: PhantomData,
        }
    }
}
