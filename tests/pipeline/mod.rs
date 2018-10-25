use async_bincode::*;
use tokio;
use tokio::prelude::*;
use tokio_tower::pipeline::{Client, Server};
//use tower_service::Service;
use tokio_tower::DirectService;
use {EchoService, PanicError, Request, Response};

mod client;

#[test]
fn integration() {
    let addr = "127.0.0.1:0".parse().unwrap();
    let rx = tokio::net::tcp::TcpListener::bind(&addr).unwrap();
    let addr = rx.local_addr().unwrap();
    let tx = tokio::net::tcp::TcpStream::connect(&addr)
        .map(AsyncBincodeStream::from)
        .map(AsyncBincodeStream::for_async)
        .map_err(PanicError::from)
        .map(|s| {
            // need to limit to one-in-flight for poll_ready to be sufficient to drive Service
            Client::with_limit(s, 1)
        });

    let rx = rx
        .incoming()
        .into_future()
        .map_err(PanicError::from)
        .map(|(stream, _)| stream.unwrap())
        .map(AsyncBincodeStream::from)
        .map(AsyncBincodeStream::for_async)
        .map_err(PanicError::from)
        .map(|stream| Server::new(stream, EchoService));

    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.spawn(
        rx.and_then(|srv| srv.map_err(PanicError::from))
            .map_err(|_| ()),
    );

    let fut = tx.map_err(PanicError::from).and_then(
        move |mut tx: Client<AsyncBincodeStream<_, Response, _, _>, _>| {
            let fut1 = tx.call(Request::new(1));

            // continue to drive the service
            tokio::spawn(
                future::poll_fn(move || tx.poll_outstanding())
                    .map_err(PanicError::from)
                    .map_err(|_| ()),
            );

            fut1.inspect(|r| r.check(1))
        },
    );
    assert!(rt.block_on(fut).is_ok());
}
