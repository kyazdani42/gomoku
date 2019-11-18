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
        &Method::GET => get(&mut response, &req),
        _ => *response.status_mut() = StatusCode::NOT_FOUND,
    }

    Box::new(future::ok(response))
}

fn get(response: &mut Response<Body>, req: &Request<Body>) {
    let path = req.uri().path();
    let params = req.uri().query();
    match path {
        "/init" => match handle_initialization(params) {
            // here i might want to send back json with serde
            Some(val) => *response.body_mut() = Body::from(val),
            None => *response.status_mut() = StatusCode::BAD_REQUEST,
        },
        _ => match get_static_asset(path) {
            Ok(val) => *response.body_mut() = Body::from(val),
            Err(_e) => *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR,
        },
    }
}

const FOLDER: &str = "./public";
const DEFAULT_FILE: &str = "./public/index.html";

fn get_static_asset(path: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut entry = fs::read_dir(FOLDER)?
        .map(|entry| entry.unwrap())
        .find(|entry| entry.file_name().into_string().unwrap() == path[1..])
        .map(|entry| format!("{:?}", entry.path()))
        .unwrap_or(DEFAULT_FILE.to_owned());
    entry.retain(|c| c != '"');

    Ok(fs::read(entry)?)
}

fn handle_initialization(params: Option<&str>) -> Option<String> {
    let params = params?
        .split("&")
        .into_iter()
        .map(|p| p.split("=").map(|s| s.to_owned()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();

    let size_param = params.iter().find(|x| x[0] == "size")?;
    if size_param.len() != 2 {
        return None;
    }
    let board_size = match size_param[1].parse::<usize>() {
        Ok(v) => v,
        Err(_) => return None,
    };
    if board_size > 25 || board_size < 19 {
        return None;
    }

    let ia_param = params.iter().find(|x| x[0] == "ia")?;
    if ia_param.len() != 2 {
        return None;
    }
    let ia = match ia_param[1].parse::<u8>() {
        Ok(v) => v,
        Err(_) => return None,
    };
    if ia != 0 && ia != 1 && ia != 2 {
        return None;
    }

    //TODO: check if ia is a player or not

    Some("hello".to_owned())
}
