extern crate futures;
extern crate hyper;

use futures::future;
use hyper::header::{HeaderValue, CONTENT_TYPE};
use hyper::rt::{Future, Stream};
use hyper::{Body, Method, Request, Response, StatusCode};
use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::{Arc, Mutex};

use crate::game::game_state::GameState;

type FutureResponse = Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send>;

pub fn router(req: Request<Body>, state: &Arc<Mutex<GameState>>) -> FutureResponse {
    let mut response = Response::new(Body::empty());

    match req.method() {
        &Method::GET => get(&mut response, &req, state),
        &Method::POST => post(&mut response, req, state),
        _ => *response.status_mut() = StatusCode::NOT_FOUND,
    }

    Box::new(future::ok(response))
}

fn post(response: &mut Response<Body>, req: Request<Body>, state: &Arc<Mutex<GameState>>) {
    let path = req.uri().path();
    match path {
        "/play" => match play(req, state) {
            Some(val) => {
                let app_json = HeaderValue::from_str("application/json").unwrap();
                response.headers_mut().insert(CONTENT_TYPE, app_json);
                *response.body_mut() = Body::from(val);
            }
            None => *response.status_mut() = StatusCode::BAD_REQUEST,
        },
        _ => *response.status_mut() = StatusCode::NOT_FOUND,
    }
}

#[derive(Deserialize, Debug)]
struct PlayBody {
    x: usize,
    y: usize,
}

fn play(req: Request<Body>, state: &Arc<Mutex<GameState>>) -> Option<String> {
    let mut new_state = state.lock().unwrap();

    // something wrong with .wait()
    let body = req.into_body().concat2().wait().unwrap().into_bytes();
    let body: PlayBody = serde_json::from_slice(&body).unwrap();

    println!("{:?}", body);

    Some("".to_owned())
}

fn get(response: &mut Response<Body>, req: &Request<Body>, state: &Arc<Mutex<GameState>>) {
    let path = req.uri().path();
    let params = req.uri().query();
    match path {
        "/init" => match handle_initialization(params, state) {
            Some(val) => {
                let app_json = HeaderValue::from_str("application/json").unwrap();
                response.headers_mut().insert(CONTENT_TYPE, app_json);
                *response.body_mut() = Body::from(val);
            }
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

fn handle_initialization(params: Option<&str>, state: &Arc<Mutex<GameState>>) -> Option<String> {
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

    // TODO: handle ia in state
    let mut state = state.lock().unwrap();
    state.init(board_size, 1);

    let data = Data {
        board: state.board.clone(),
        player: state.player,
        winner: state.winner,
    };
    match serde_json::to_string(&data) {
        Ok(json) => Some(json),
        Err(_) => None,
    }
}

#[derive(Serialize, Deserialize)]
struct Data {
    board: Vec<Vec<u8>>,
    player: u8,
    winner: u8,
}
