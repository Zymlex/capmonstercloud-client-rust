use std::error::Error;

use hyper::service::service_fn;
use hyper_util::{
    rt::{TokioExecutor, TokioIo},
    server::conn::auto::Builder,
};
use logic::handle_request;
use tokio::{net::TcpListener, task::JoinSet};

use super::APIRootUrls;

mod logic;
mod responses;
mod verify;

pub(super) async fn start_server<'a>(
    ip_port: &'a str,
    _api_roots: &'a APIRootUrls<'a>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let tcp_listener = TcpListener::bind(ip_port).await.unwrap();

    let mut join_set = JoinSet::new();

    loop {
        let (stream, addr) = match tcp_listener.accept().await {
            Ok(x) => x,
            Err(e) => {
                eprintln!("failed to accept connection: {e}");
                continue;
            }
        };

        join_set.spawn(async move {
            println!("handling a request from {addr}");

            Builder::new(TokioExecutor::new())
                .serve_connection(TokioIo::new(stream), service_fn(handle_request))
                .await
                .inspect_err(|e: &Box<dyn Error + Send + Sync>| {
                    eprintln!("error servicing {addr}: {e}")
                })
                .unwrap();
        });
    }

    // If you add a method for breaking the above loop (i.e. graceful shutdown),
    // then you may also want to wait for all existing connections to finish
    // being served before terminating the program, which can be done like this:
    //
    // while let Some(_) = join_set.join_next().await {}
}
