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

mod game_state;
mod win;
mod free_three;
mod utils;
mod capture;

pub use win::*;
pub use free_three::*;
pub use capture::*;
pub use utils::*;
pub use game_state::*;

