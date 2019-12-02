pub const JOINED_ACTIONS: [&str; 4] = [
    "left|right",
    "top|bot",
    "bot_left|top_right",
    "top_left|bot_right",
];

pub const ACTIONS: [&str; 8] = [
    "bot_left",
    "top_right",
    "left",
    "right",
    "top",
    "bot",
    "top_left",
    "bot_right",
];

mod board;
mod game_state;

pub use board::*;
pub use game_state::*;

