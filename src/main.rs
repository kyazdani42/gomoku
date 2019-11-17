extern crate futures;
extern crate hyper;

use futures::{future, Stream};
use hyper::rt::Future;
use hyper::service::service_fn;
use hyper::{Body, Request, Response, Server};
use hyper::{Method, StatusCode};

fn main() {
    let port = 3001;
    let localhost = [127, 0, 0, 1];
    let addr = (localhost, port).into();

    let server = match Server::try_bind(&addr) {
        Ok(srv) => srv,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    }
    .serve(|| service_fn(router))
    .map_err(|e| eprintln!("server error: {}", e));

    hyper::rt::run(server);
}

type FutureResponse = Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send>;

fn router(req: Request<Body>) -> FutureResponse {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("return the index.html file");
        }
        (&Method::GET, "/index.js") => {
            *response.body_mut() = Body::from("return the index.js file");
        }
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    }

    Box::new(future::ok(response))
}

// fn router_example(req: Request<Body>) -> FutureResponse {
//     let mut response = Response::new(Body::empty());
//
//     match (req.method(), req.uri().path()) {
//         (&Method::GET, "/") => {
//             *response.body_mut() = Body::from("Try POSTing data to /echo");
//         }
//         (&Method::POST, "/echo") => {
//             *response.body_mut() = req.into_body();
//         }
//         (&Method::POST, "/echo/uppercase") => {
//             let mapping = req.into_body().map(|chunk| {
//                 chunk
//                     .iter()
//                     .map(|byte| byte.to_ascii_uppercase())
//                     .collect::<Vec<u8>>()
//             });
//             *response.body_mut() = Body::wrap_stream(mapping);
//         }
//         (&Method::POST, "/echo/reverse") => {
//             let reversed_mapping = req.into_body().concat2().map(move |chunk| {
//                 let body = chunk.iter().rev().cloned().collect::<Vec<u8>>();
//                 *response.body_mut() = Body::from(body);
//                 response
//             });
//             return Box::new(reversed_mapping);
//         }
//         _ => {
//             *response.status_mut() = StatusCode::NOT_FOUND;
//         }
//     };
//
//     Box::new(future::ok(response))
// }
