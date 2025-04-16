use std::convert::Infallible;

use bytes::Bytes;
use http::{Method, Request, Response};
use http_body_util::{BodyExt, Full};
use hyper::body::{Body, Incoming};

use crate::{ImageToTextTask, tests::CLIENT_KEY};

use super::responses::*;

pub(super) async fn handle_request(
    req: Request<Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    let uri = req.uri();
    println!("{:?}", req);

    let res = match (req.method(), uri.path()) {
        (&Method::GET, "/api/useragent/actual") => resp(UA),
        (&Method::POST, "/getBalance") => resp(BALANCE_RESP),
        (&Method::POST, "/createTask") => {
            let max_size = req.body().size_hint().upper().unwrap_or(u64::MAX);
            if max_size > 1024 * 64 {
                let mut resp = Response::builder()
                    .status(hyper::StatusCode::PAYLOAD_TOO_LARGE)
                    .body("Body too big".into())
                    .unwrap();
                return Ok(resp);
            }

            let req_body = req.collect().await.unwrap().to_bytes();

            // let b = ;

            // match serde_json::from_slice::<CreateTask<ImageToTextTask>>(&req_body) {
            //     Ok(r) => {
            //         if r.clientKey != CLIENT_KEY {
            //             return Ok(full(WRONG_CLIENT_KEY));
            //         }

            //         if r.softId != "60" {
            //             return Ok(full("WRONG SOFT ID"));
            //         }

            //         // r.task.
            //     }
            //     Err(e) => {
            //         return Ok(Response::builder()
            //             .status(503)
            //             .body(Full::new(e.into()))
            //             .unwrap());
            //     }
            // }
            // CreateTask
            Response::builder()
                .status(404)
                .body(Bytes::default())
                .unwrap()
        }
        (&Method::POST, "/getTaskResult") => Response::builder()
            .status(404)
            .body(Bytes::default())
            .unwrap(),
        _ => Response::builder()
            .status(404)
            .body(Bytes::default())
            .unwrap(),
    };

    Ok(res)
}

fn resp<T: Into<Bytes>>(chunk: T) -> Response<Bytes> {
    Response::new(chunk.into())
}
