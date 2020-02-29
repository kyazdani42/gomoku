use std::sync::{Arc, Mutex};

use hyper::rt::Future;
use hyper::service::service_fn;
use hyper::Server;

// mod algorithm;
mod lib;
mod router;

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

    let state = Arc::new(Mutex::new(lib::State::new()));

    let server = server
        .serve(move || {
            let state = state.clone();
            service_fn(move |req| router::router(req, &state))
        })
        .map_err(|e| eprintln!("server error: {}", e));

    println!("server running on \x1b[1mhttp://{}\x1b[0m", addr);
    hyper::rt::run(server);
}
