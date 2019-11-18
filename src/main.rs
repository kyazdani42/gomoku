extern crate hyper;

use hyper::rt::Future;
use hyper::service::service_fn;
use hyper::Server;

// mod game_rules;
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
    }
    .serve(|| service_fn(router::router))
    .map_err(|e| eprintln!("server error: {}", e));

    hyper::rt::run(server);
}
