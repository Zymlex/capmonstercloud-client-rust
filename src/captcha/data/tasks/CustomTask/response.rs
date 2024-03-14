use serde::Deserialize;

// TODO FunCaptchaTaskProxylessResp

// impl TaskRespTrait for FunCaptchaTaskProxylessResp {}

// #[derive(Deserialize, Clone, Debug)]
// pub struct CustomTask {
//     domains: [],
// }

#[derive(Deserialize, Clone, Debug)]
pub struct Domain {
    site: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct DataDomeCookie {
    
}