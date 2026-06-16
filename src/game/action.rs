use serde::{Deserialize, Serialize};

use crate::game::game_logic::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Action {
    Up,
    Down,
    Left,
    Right,
    Wait,
}

impl Action {
    pub fn to_point(&self) -> Point {
        match self {
            Action::Up => (0, -1),
            Action::Down => (0, 1),
            Action::Right => (1, 0),
            Action::Left => (-1, 0),
            Action::Wait => (0, 0),
        }
    }
}

