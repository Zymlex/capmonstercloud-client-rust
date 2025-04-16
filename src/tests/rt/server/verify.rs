// use std::convert::Infallible;

use http::{Response, StatusCode};

use super::responses::BALANCE_RESP;

pub(super) async fn verify_balance<'a>(
    client_key: &'a str,
) -> Result<Response<&'static str>, Response<&'static str>> {
    match client_key {
        "00000" => Ok(Response::builder()
            .status(StatusCode::OK)
            .body(BALANCE_RESP)
            .unwrap()),
        _ => {
            return Err(Response::builder()
                .status(StatusCode::OK)
                .body(BALANCE_RESP)
                .unwrap());
        }
    }
}
