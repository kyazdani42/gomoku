extern crate hyper;

use hyper::rt::Future;
use hyper::service::service_fn;
use hyper::Server;
use std::sync::{Arc, Mutex};

mod game;
mod router;

use game::game_state::GameState;

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
    };

    let state = Arc::new(Mutex::new(GameState::new()));

    let server = server
        .serve(move || {
            let state = state.clone();
            service_fn(move |req| router::router(req, &state))
        })
        .map_err(|e| eprintln!("server error: {}", e));

    println!("server running on \x1b[1mhttp://{}\x1b[0m", addr);
    hyper::rt::run(server);
}
