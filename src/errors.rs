use reqwest::Error;

#[derive(Debug)]
pub enum CapMonsterCloudClientError {
    BuildUriError(Error),
}

#[derive(Debug)]
pub enum ClientOptionsError {}

// impl From<InvalidUri> for ClientOptionsError {
//     fn from(e: InvalidUri) -> Self {
//         ClientOptionsError::InvalidUri(e)
//     }
// }

#[derive(Debug)]
pub(crate) enum HttpClientError {
    // PostRequestError(Error),
    // PostResponseError(Error),
}

#[derive(Debug)]
pub(crate) enum ResponseError {}

#[derive(Debug)]
pub enum TaskCreationError {}

#[derive(Debug)]
pub(crate) enum TaskInvalid {
    TaskInvalid,
    RequestsLimitReached
}

#[derive(Debug)]
pub(crate) enum GetTaskError {}