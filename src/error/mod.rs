use crate::*;
use reqwest::StatusCode;

#[derive(Debug)]
pub enum CapMonsterCloudClientError {
    InputOptionsError(OptionsError),
    ClientImplError(RequestCreatorError),
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
pub enum SvcResponseError {
    GettingResultError(
        SvcRespStructError,
    ),
    SvcReturnErrorCode(SvcErrorResp),
}

#[derive(Debug)]
pub enum RequestCreatorError {
    HttpClientCreationError(reqwest::Error),
    PostRequestError(reqwest::Error),
    NonSuccessResponseStatus(StatusCode),
    ResponseToStringError(reqwest::Error),
    SerializationError(serde_json::Error),
    DeserializationSuccessError(serde_json::Error),
    DeserializationErrorError(serde_json::Error),
}

#[derive(Debug)]
pub enum SvcRespStructError {
    _ReportBug,
    _ReportBugSuccessResponseWithoutData,
    GetTaskError(GetTaskError)
}

#[derive(Debug)]
pub enum GetBalanceError {
    RequestError(RequestCreatorError),
    SvcResponseError(SvcResponseError),
}

#[derive(Debug)]
pub enum CMCTaskError {
    TaskCreationError(RequestCreatorError),
    CreateTaskGetResultError(SvcResponseError),
    GetTaskResultError(TaskResultError),
    TaskResultError(SvcResponseError),
}

#[derive(Debug)]
pub enum SolveError {
    TaskCreationError(CMCTaskError),
    GetTaskResultErrorInLoop(TaskResultError),
}

#[derive(Debug)]
pub enum TaskResultError {
    RequestError(RequestCreatorError),
    BadStatusCode(StatusCode),
    RequestsLimitReached,
    SerializationError(serde_json::Error),
    GetResultFailed(RequestCreatorError),
    GetResultTotalTimeout,
    ReadyTaskWithoutSolution,
    SvcResponseError(SvcResponseError),
}

#[derive(Debug)]
pub enum GetTaskError {
    Processing,
    ReadyTaskWithoutSolution,
    UnsupportedStatus(String),
}
