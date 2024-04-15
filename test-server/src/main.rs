#![deny(warnings)]

use std::task::{Context, Poll};

use futures_util::future;
use hyper::service::Service;
use hyper::{Body, Method, Request, Response, Server};
use hyper::header::{CONTENT_LENGTH, CONTENT_TYPE};
use hyper_routing::{Route, RouterBuilder, RouterService};

pub struct MakeSvc;

impl<T> Service<T> for MakeSvc {
    type Response = RouterService;
    type Error = std::io::Error;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Ok(()).into()
    }

    fn call(&mut self, _: T) -> Self::Future {
        future::ok(router_service())
    }
}

fn request_handler(_: Request<Body>) -> Response<Body> {
    let body = "Hello World";
    Response::builder()
        .header(CONTENT_LENGTH, body.len() as u64)
        .header(CONTENT_TYPE, "text/plain")
        .body(Body::from(body))
        .expect("Failed to construct the response")
}

fn router_service() -> RouterService {
    let router = RouterBuilder::new()
        .add(Route::get("/hello").using(request_handler))
        .add(Route::from(Method::PATCH, "/world").using(request_handler))
        .build();

    RouterService::new(router)
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // We'll bind to 127.0.0.1:3000
    let addr = "0.0.0.0:3000".parse().unwrap();

    let server =
        Server::bind(&addr).serve(MakeSvc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}