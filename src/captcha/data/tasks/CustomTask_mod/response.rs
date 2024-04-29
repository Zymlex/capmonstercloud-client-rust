use crate::TaskRespTrait;
use serde::Deserialize;
use std::collections::HashMap;

impl TaskRespTrait for CustomTaskResp {}

#[derive(Deserialize, Clone, Debug)]
pub struct CustomTaskResp {
    pub domains: CustomTaskDomains,
}

#[derive(Deserialize, Clone, Debug)]
pub struct CustomTaskDomains {
    #[serde(flatten)]
    pub site: HashMap<String, DataDomeCookies>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct DataDomeCookies {
    pub datadome: String,
}
