extern crate hyper;

use hyper::header::CONTENT_TYPE;
use hyper::rt::{lazy, Future, Stream};
use hyper::{Body, Error, Method, Request, Response, StatusCode};
use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::{Arc, Mutex};

use crate::game::game_state::GameState;

static INTERNAL_SERVER_ERROR: &[u8] = b"Internal Server Error";
static NOTFOUND: &[u8] = b"Not Found";
static BADREQUEST: &[u8] = b"Bad Request";

type FutureResponse = Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send>;

pub fn router(req: Request<Body>, state: &Arc<Mutex<GameState>>) -> FutureResponse {
    let response = match req.method() {
        &Method::GET => get(req, state),
        &Method::POST => post(req, state),
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(NOTFOUND.into())
            .unwrap()),
    };

    Box::new(lazy(|| response))
}

fn post(req: Request<Body>, state: &Arc<Mutex<GameState>>) -> Result<Response<Body>, Error> {
    match req.uri().path() {
        "/play" => play(req, state),
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(NOTFOUND.into())
            .unwrap()),
    }
}

#[derive(Deserialize, Debug)]
struct PlayBody {
    x: usize,
    y: usize,
}

fn play(req: Request<Body>, state: &Arc<Mutex<GameState>>) -> Result<Response<Body>, Error> {
    // TODO: find a way to do this synchronously or handle asynchronousity somewhere
    let body = req.into_body().concat2().map(|chunk| chunk.into_bytes());
    // let mut data: PlayBody = serde_json::from_slice(&body.to_vec()).unwrap();

    let response = Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, "application/json")
        .body(Body::from("okay"))
        .unwrap();

    Ok(response)
}
// let mut state = state.lock().unwrap();
// let data = Data {
//     board: state.board.clone(),
//     player: state.player,
//     winner: state.winner,
// };
// let json_data = match serde_json::to_string(&data) {
//     Ok(json) => json,
//     Err(_) => return *response.status_mut() = StatusCode::BAD_REQUEST,
// };
//
// let app_json = HeaderValue::from_str("application/json").unwrap();
// response.headers_mut().insert(CONTENT_TYPE, app_json);
// *response.body_mut() = Body::from(json_data);

fn get(req: Request<Body>, state: &Arc<Mutex<GameState>>) -> Result<Response<Body>, Error> {
    let uri = req.uri();
    match uri.path() {
        "/init" => match handle_initialization(uri.query(), state) {
            Some(val) => Ok(Response::builder()
                .header(CONTENT_TYPE, "application/json")
                .status(StatusCode::OK)
                .body(Body::from(val))
                .unwrap()),
            None => Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(BADREQUEST.into())
                .unwrap()),
        },
        _ => match get_static_asset(uri.path()) {
            Ok(val) => Ok(Response::builder()
                .status(StatusCode::OK)
                .body(Body::from(val))
                .unwrap()),
            Err(_e) => Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(INTERNAL_SERVER_ERROR.into())
                .unwrap()),
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
