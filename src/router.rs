extern crate futures;
extern crate hyper;

use futures::{future, Stream};
use hyper::rt::Future;
use hyper::{Body, Request, Response};
use hyper::{Method, StatusCode};

type FutureResponse = Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send>;

pub fn router(req: Request<Body>) -> FutureResponse {
    let mut response = Response::new(Body::empty());

    match req.method() {
        &Method::GET => get(&mut response, req.uri().path()),
        _ => *response.status_mut() = StatusCode::NOT_FOUND,
    }

    Box::new(future::ok(response))
}

fn get(response: &mut Response<Body>, path: &str) {
    if path == "/" {
        match std::fs::read_to_string("./public/index.html") {
            Ok(file) => *response.body_mut() = Body::from(file),
            Err(_e) => *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR,
        }
    } else if path == "/index.js" {
        match std::fs::read_to_string("./public/index.js") {
            Ok(file) => *response.body_mut() = Body::from(file),
            Err(_e) => *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR,
        }
    } else {
        *response.status_mut() = StatusCode::NOT_FOUND;
    }
}

// fn router_example(req: Request<Body>) -> FutureResponse {
//     let mut response = Response::new(Body::empty());
//
//     match (req.method(), req.uri().path()) {
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
//     };
//
//     Box::new(future::ok(response))
// }
