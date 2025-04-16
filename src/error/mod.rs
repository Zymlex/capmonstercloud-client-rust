use crate::*;
use reqwest::StatusCode;

#[derive(Debug)]
pub enum CapMonsterCloudClientError {
    ClientImpl(RequestCreatorError),
}

#[derive(Debug)]
pub enum UrlsError {
    UrlParse(String), // 'String' because `url::parser::ParseError` requires explicit crate in dependencies
}

#[derive(Debug)]
pub enum SvcResponseError {
    GettingResult(SvcRespStructError),
    SvcReturnErrorCode(SvcErrorResp),
}

#[derive(Debug)]
pub enum RequestCreatorError {
    Urls(UrlsError), // 'String' because `url::parser::ParseError` requires explicit crate in dependencies
    HttpClientCreation(reqwest::Error),
    GetRequest(reqwest::Error),
    PostRequest(reqwest::Error),
    NonSuccessResponseStatus(StatusCode),
    ResponseToString(reqwest::Error),
    Serialization(serde_json::Error),
    DeserializationSvcSuccess(serde_json::Error),
    DeserializationSvcError(serde_json::Error),
}

#[derive(Debug)]
pub enum SvcRespStructError {
    _ReportBug,
    _ReportBugSuccessResponseWithoutData,
    GetTaskError(GetTaskError),
}

#[derive(Debug)]
pub enum GetUserAgentError {
    Request(RequestCreatorError),
    SvcResponse(SvcResponseError),
}

#[derive(Debug)]
pub enum GetBalanceError {
    Request(RequestCreatorError),
    SvcResponse(SvcResponseError),
}

#[derive(Debug)]
pub enum CMCTaskError {
    TaskCreation(RequestCreatorError),
    CreateTaskGetResult(SvcResponseError),
    GetTaskResult(TaskResultError),
    TaskResult(SvcResponseError),
}

#[derive(Debug)]
pub enum SolveError {
    TaskCreation(CMCTaskError),
    GetTaskResultInLoop(TaskResultError),
}

#[derive(Debug)]
pub enum TaskResultError {
    Request(RequestCreatorError),
    BadStatusCode(StatusCode),
    RequestsLimitReached,
    Serialization(serde_json::Error),
    GetResult(RequestCreatorError),
    GetResultTotalTimeout,
    ReadyTaskWithoutSolution,
    SvcResponse(SvcResponseError),
}

#[derive(Debug)]
pub enum GetTaskError {
    Processing,
    ReadyTaskWithoutSolution,
    UnsupportedStatus(String),
}
