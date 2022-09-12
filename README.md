# Hyper Router [![Build Status](https://travis-ci.com/tsharp/hyper-routing.svg?branch=master)](https://travis-ci.com/tsharp/hyper-routing)

This cargo is a small extension to the great Hyper HTTP library. It basically is
adds the ability to define routes to request handlers and then query for the handlers
by request path.

[API Documentation](https://docs.rs/hyper-routing/latest/hyper_routing/)

## Usage

To use the library just add:

```toml
hyper-routing = "0.6.1"
hyper = { version = "0.14", features = ["full"] }
tokio = { version = "1", features = ["full"] }
futures-util = "0.3"
pretty_env_logger = "0.4"
```

to your dependencies.

```rust
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
```

This code will start Hyper server and add use router to find handlers for request.
We create the `Route` so that when we visit path `/greet` the `basic_handler` handler
will be called.

## Things to note

- you can specify paths as regular expressions so you can match every path you please.
- If you have request matching multiple paths the one that was first `add`ed will be chosen.
- ~~This library is in an early stage of development so there may be breaking changes comming.~~ -
  it seems that the library is quite popular so I'm not going to do compatibility breaking changes.

# Further Development

- add the ability to distinguish requests by query parameters.

# Waiting for your feedback

I've created this little tool to help myself learn Rust and to avoid using big frameworks
like Iron or rustful. I just want to keep things simple.

Obviously I could make some errors or bad design choices so I'm waiting for your feedback!
Please contact me at moriturius at GMail. You may also create an issue at [project's bug tracker](https://github.com/tsharp/hyper-routing/issues).
