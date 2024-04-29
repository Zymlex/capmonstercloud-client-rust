#![allow(clippy::enum_variant_names)]

use reqwest::StatusCode;
use crate::error::response_error::ResponseError;

pub mod response_error;

#[derive(Debug)]
pub enum CapMonsterCloudClientError {
    InputOptionsError(OptionsError),
    ClientImplError(ClientImplError),
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
pub enum ClientImplError {
    HttpClientCreationError(reqwest::Error),
}

#[derive(Debug)]
pub enum SvcRequestError {
    SerializeError(serde_json::Error),
    PostRequestError(reqwest::Error),
    NonSuccessRespStatus(StatusCode),
}

#[derive(Debug)]
pub enum SvcResponseError {
    SerializeError(serde_json::Error),
    DeserializeError(serde_json::Error),
    RespToStringError(reqwest::Error),
}

#[derive(Debug)]
pub enum DeserializeError {
    SerializeError(serde_json::Error),
    DeserializeError(serde_json::Error),
    RespToStringError(reqwest::Error),
}

#[derive(Debug)]
pub enum GetBalanceError {
    SerializeError(serde_json::Error),
    DeserializeError(SvcResponseError),
    RequestError(SvcRequestError),
    GetResultError(ResponseError),
}

#[derive(Debug)]
pub enum SolveError {
    TaskCreationError(TaskCreationError),
    TaskResultError(TaskResultError),
}

#[derive(Debug)]
pub enum TaskCreationError {
    RequestError(SvcRequestError),
    SerializeError(serde_json::Error),
    DeserializeError(SvcResponseError),
    InvalidResponse(ResponseError),
}

#[derive(Debug)]
pub enum TaskResultError {
    InvalidResponse(&'static str),
    RequestError(SvcRequestError),
    RequestsLimitReached,
    SerializeError(serde_json::Error),
    DeserializeError(DeserializeError),
    GetResultTimeout,
}

#[derive(Debug)]
pub enum GetTaskError {
    Processing,
    ReadyTaskWithoutSolution,
}
