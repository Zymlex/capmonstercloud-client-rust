use crate::error::GetTaskError;

#[derive(Debug)]
pub struct ResponseError {
    reason: ResponseErrorVariants,
    #[cfg(feature = "reserve_response_body")]
    body: String,
}

impl<'a> ResponseError {
    pub(crate) fn new(
        reason: ResponseErrorVariants,
        #[cfg(feature = "reserve_response_body")] body: String,
    ) -> Self {
        Self {
            reason,
            #[cfg(feature = "reserve_response_body")]
            body,
        }
    }
    
    #[cfg(feature = "reserve_response_body")]
    pub fn get_body(&'a self) -> &'a str {
        &self.body
    }
}

#[derive(Debug)]
pub enum ResponseErrorVariants {
    Error(ErrorInfo),
    SuccessResponseWithoutData,
    GetTaskError(GetTaskError),
}

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct ErrorInfo {
    errorCode: Option<String>,
    errorDescription: Option<String>,
}

#[allow(non_snake_case)]
impl ErrorInfo {
    pub(crate) fn new(errorCode: Option<String>, errorDescription: Option<String>) -> Self {
        Self {
            errorCode,
            errorDescription,
        }
    }
}