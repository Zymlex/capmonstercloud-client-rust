#![allow(clippy::enum_variant_names)]

use crate::error::response_error::ResponseError;

pub mod response_error;

#[derive(Debug)]
pub enum CapMonsterCloudClientError {
    InputOptionsError(OptionsError),
}

#[derive(Debug)]
pub enum OptionsError {
    UrlParseError(String), // 'String' because `url::parser::ParseError` requires explicit crate in dependencies
}

impl From<OptionsError> for CapMonsterCloudClientError {
    fn from(e: OptionsError) -> Self {
        Self::InputOptionsError(e)
    }
}

#[derive(Debug)]
pub enum HttpClientError {
    PostRequestError(reqwest::Error),
    RawRespToTextError(reqwest::Error),
    TextToStructError(serde_json::Error),
}

#[derive(Debug)]
pub enum GetBalanceError {
    StructToJsonError(serde_json::Error),
    RequestError(HttpClientError),
    GetResultError(ResponseError),
}

#[derive(Debug)]
pub enum SolveError {
    TaskCreationError(TaskCreationError),
    TaskResultError(TaskResultError),
}

#[derive(Debug)]
pub enum TaskCreationError {
    RequestError(HttpClientError),
    StructToJsonError(serde_json::Error),
    InvalidResponse(ResponseError),
}

#[derive(Debug)]
pub enum TaskResultError {
    InvalidResponse(&'static str),
    RequestError(HttpClientError),
    RequestsLimitReached,
    GetResultTimeout,
    StructToJsonError(serde_json::Error),
}

#[derive(Debug)]
pub enum GetTaskError {
    Processing,
    ReadyTaskWithoutSolution,
}
