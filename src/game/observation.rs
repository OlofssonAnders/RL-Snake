use std::collections::LinkedList;
use crate::game::game_logic::Point;
use crate::game::action::Action;

pub struct Observation {
    pub snake_head: Point,
    pub snake_body: LinkedList<Point>,
    pub fruit: Point,
    pub score: i32,
    pub game_over: bool,
    pub squares: i8,
    pub speed: f32,
    pub prev_dir: Action,
}
