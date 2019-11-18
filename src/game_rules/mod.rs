pub mod aligned;
pub mod moves;
pub mod rules;
pub mod captures;

// use game_rules::rules::place_piece;
//
// const LINE_LENGTH: usize = 19;
// const BOARD_LENGTH: usize = LINE_LENGTH * LINE_LENGTH;
//
// fn switch_player(player: i32) -> i32 {
//     if player == 1 {
//         2
//     } else {
//         1
//     }
// }
//
// fn play() {
//     let mut board: [i32; BOARD_LENGTH] = [0; BOARD_LENGTH];
//     let mut game_running: bool = true;
//     let mut player = 1;
//
//     while game_running {
//         place_piece(&mut board, 12, 1);
//         game_running = false;
//     }
//     player = switch_player(player);
//     eprintln!("board : {:?}", board[12]);
//     eprintln!("player : {}", player);
// }
