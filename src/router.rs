extern crate futures;
extern crate hyper;

use futures::future;
use hyper::rt::Future;
use hyper::{Body, Request, Response};
use hyper::{Method, StatusCode};
use std::fs;

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
    match path {
        "/all_the_route_with_no_hanlding_will_try_to_access_the_public_files" => {}
        // might want to do a handle_static that sets the response itself and handles error
        // in case image detection doesnt work as expected
        _ => match get_static_asset(path) {
            Ok(val) => *response.body_mut() = Body::from(val),
            Err(_e) => *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR,
        },
    }
}

const FOLDER: &str = "./public";
const DEFAULT_FILE: &str = "./public/index.html";

fn get_static_asset(path: &str) -> Result<String, std::io::Error> {
    for entry in fs::read_dir(FOLDER)? {
        let direntry = entry?;
        let mut filepath = format!("{:?}", direntry.path());
        filepath.retain(|c| c != '"');

        let filename = direntry.file_name().into_string().unwrap();
        let mut extension = "/".to_owned();
        extension.push_str(&filename);

        // need to find how to detect it is an image and send back properly

        if extension == path {
            return Ok(std::fs::read_to_string(filepath)?);
        }
    }
    Ok(std::fs::read_to_string(DEFAULT_FILE)?)
}

