extern crate futures;
extern crate hyper;

use futures::future;
use hyper::rt::Future;
use hyper::{Body, Request, Response};
use hyper::{Method, StatusCode};
use std::fs;
use std::io::Read;

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
    let mut entry = fs::read_dir(FOLDER)?
        .map(|entry| entry.unwrap())
        .find(|entry| entry.file_name().into_string().unwrap() == path[1..])
        .map(|entry| format!("{:?}", entry.path()))
        .unwrap_or(DEFAULT_FILE.to_owned());
    entry.retain(|c| c != '"');

    // if its an image, do something else
    if entry.ends_with(".jpg") || entry.ends_with(".jpeg") {
        println!("its an image");
        let file = fs::read(&entry)?
            .iter()
            .map(|chunk| chunk.to_string())
            .collect::<Vec<String>>();
        return Ok(file.join(""));
    }
    Ok(fs::read_to_string(entry)?)
}
