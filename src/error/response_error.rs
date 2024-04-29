use crate::error::GetTaskError;

#[derive(Debug)]
pub enum ResponseError {
    NotSuccessStatus,
    Error,
    SuccessResponseWithoutData,
    GetTaskError(GetTaskError),
}