extern crate hyper;

use hyper::header::{ACCESS_CONTROL_ALLOW_ORIGIN, CONTENT_TYPE};
use hyper::rt::{lazy, Future};
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
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(NOTFOUND.into())
            .unwrap()),
    };

    Box::new(lazy(|| response))
}

fn get(req: Request<Body>, state: &Arc<Mutex<GameState>>) -> Result<Response<Body>, Error> {
    let uri = req.uri();
    match uri.path() {
        "/init" => match handle_initialization(uri.query(), state) {
            Some(val) => Ok(Response::builder()
                .header(CONTENT_TYPE, "application/json")
                .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
                .status(StatusCode::OK)
                .body(Body::from(val))
                .unwrap()),
            None => Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(BADREQUEST.into())
                .unwrap()),
        },
        "/play" => match play(uri.query(), state) {
            Some(val) => Ok(Response::builder()
                .header(CONTENT_TYPE, "application/json")
                .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
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

#[derive(Serialize, Deserialize)]
struct ResponseData {
    board: Vec<Vec<u8>>,
    player: u8,
    winner: u8,
}

fn play(params: Option<&str>, state: &Arc<Mutex<GameState>>) -> Option<String> {
    let params = get_params(params)?;
    let line_param = find_param(&params, "line")?;
    let line = parse_param(&line_param)?;

    let col_param = find_param(&params, "col")?;
    let col = parse_param(&col_param)?;

    let mut state = state.lock().unwrap();
    state.play(line, col);

    get_response_data(&state)
}

fn handle_initialization(params: Option<&str>, state: &Arc<Mutex<GameState>>) -> Option<String> {
    let params = get_params(params)?;
    let size_param = find_param(&params, "size")?;

    let board_size = parse_param(&size_param)?;
    if board_size > 25 || board_size < 19 {
        return None;
    }

    let ia_param = find_param(&params, "ia")?;
    let ia = parse_param(&ia_param)? as u8;

    if ia != 0 && ia != 1 && ia != 2 {
        return None;
    }

    // TODO: handle ia in state
    let mut state = state.lock().unwrap();
    state.init(board_size, 1);

    get_response_data(&state)
}

fn get_response_data(state: &GameState) -> Option<String> {
    let data = ResponseData {
        board: state.board.clone(),
        player: state.player,
        winner: state.winner,
    };
    match serde_json::to_string(&data) {
        Ok(json) => Some(json),
        Err(_) => None,
    }
}

fn parse_param(param: &str) -> Option<usize> {
    match param.parse::<usize>() {
        Ok(v) => Some(v),
        Err(_) => None,
    }
}

fn get_params(params: Option<&str>) -> Option<Vec<Vec<String>>> {
    let params = params?
        .split("&")
        .into_iter()
        .map(|p| p.split("=").map(|s| s.to_owned()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();

    Some(params)
}

fn find_param(params: &Vec<Vec<String>>, name: &str) -> Option<String> {
    let param = params.iter().find(|x| x[0] == name)?;
    if param.len() != 2 {
        return None;
    }
    Some(param[1].clone())
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
