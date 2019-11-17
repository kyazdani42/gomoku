extern crate hyper;

use hyper::rt::Future;
use hyper::service::service_fn_ok;
use hyper::{Body, Request, Response, Server};

pub fn run_server() {
    let server = Server::try_bind(&([127, 0, 0, 1], 3000).into()).serve(make_service);

    // Prepare some signal for when the server should start shutting down...
    let (tx, rx) = tokio::sync::oneshot::channel::<()>();
    let graceful = server.with_graceful_shutdown(async {
        rx.await.ok();
    });

    // Await the `server` receiving the signal...
    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }

    // And later, trigger the signal by calling `tx.send(())`.
    let _ = tx.send(());
}
