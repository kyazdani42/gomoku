//mod game_rules;
//use game_rules::rules::place_piece;
//static LINE_LENGTH: usize = 19;
//static BOARD_LENGTH: usize = LINE_LENGTH * LINE_LENGTH;

//fn main() {
//    play()
//}

//fn switch_player(player: i32) -> i32 {
//    if player == 1 { 2 } else { 1 }
//}
//
//fn play() {
//    let mut board: [i32; BOARD_LENGTH] = [0; BOARD_LENGTH];
//    let mut game_running: bool = true;
//    let mut player = 1;
//
//    while game_running {
//        place_piece(&mut board, 12, 1);
//        game_running = false;
//    }
//    player = switch_player(player);
//    eprintln!("board : {:?}", board[12]);
//    eprintln!("player : {}", player);
//}
extern crate hyper;

use hyper::rt::Future;
use hyper::service::service_fn;
use hyper::Server;

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

