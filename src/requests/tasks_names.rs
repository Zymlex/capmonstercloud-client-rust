use std::fmt::Debug;
use crate::requests::tasks_data::*;

pub(crate) trait TaskReqTrait: Clone + Debug {
    fn get_task_name(&self) -> &'static str;
}

impl<'a> TaskReqTrait for ImageToTextTaskReq<'a> {
    fn get_task_name(&self) -> &'static str {
        "ImageToTextTask"
    }
}

impl<'a> TaskReqTrait for NoCaptchaTaskProxylessReq<'a> {
    fn get_task_name(&self) -> &'static str {
        "NoCaptchaTaskProxyless"
    }
}

impl<'a> TaskReqTrait for NoCaptchaTaskReq<'a> {
    fn get_task_name(&self) -> &'static str {
        "NoCaptchaTask"
    }
}

impl<'a> TaskReqTrait for RecaptchaV3TaskProxylessReq<'a> {
    fn get_task_name(&self) -> &'static str {
        "RecaptchaV3TaskProxyless"
    }
}

impl<'a> TaskReqTrait for RecaptchaV2EnterpriseTaskReq<'a> {
    fn get_task_name(&self) -> &'static str {
        "RecaptchaV2EnterpriseTask"
    }
}

impl<'a> TaskReqTrait for RecaptchaV2EnterpriseTaskProxylessReq<'a> {
    fn get_task_name(&self) -> &'static str {
        "RecaptchaV2EnterpriseTaskProxyless"
    }
}

impl<'a> TaskReqTrait for FunCaptchaTaskReq<'a> {
    fn get_task_name(&self) -> &'static str {
        "FunCaptchaTask"
    }
}

impl<'a> TaskReqTrait for FunCaptchaTaskProxylessReq<'a> {
    fn get_task_name(&self) -> &'static str {
        "FunCaptchaTaskProxyless"
    }
}

impl<'a> TaskReqTrait for HCaptchaTaskReq<'a> {
    fn get_task_name(&self) -> &'static str {
        "HCaptchaTask"
    }
}

impl<'a> TaskReqTrait for HCaptchaTaskProxylessReq<'a> {
    fn get_task_name(&self) -> &'static str {
        "HCaptchaTaskProxyless"
    }
}

impl<'a> TaskReqTrait for GeeTestTaskReq<'a> {
    fn get_task_name(&self) -> &'static str {
        "GeeTestTask"
    }
}

impl<'a> TaskReqTrait for GeeTestTaskProxylessReq<'a> {
    fn get_task_name(&self) -> &'static str {
        "GeeTestTaskProxyless"
    }
}

impl<'a> TaskReqTrait for TurnstileTaskReq<'a> {
    fn get_task_name(&self) -> &'static str {
        "TurnstileTask"
    }
}

impl<'a> TaskReqTrait for TurnstileTaskProxylessReq<'a> {
    fn get_task_name(&self) -> &'static str {
        "TurnstileTaskProxyless"
    }
}
