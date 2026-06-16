use crate::game::{Action, Observation};
use crate::controllers::Controller;
use macroquad::prelude::*;


pub struct HumanController {
    current_action: Action,
}

impl Controller for HumanController {
    fn choose_action(&mut self, _obs: &Observation) -> Action {
        match self.read_input() {
            Some(action) => {
                self.current_action = action;
                self.current_action
            }
            None => self.current_action,
        }
    }
}

impl HumanController {
    pub fn new() -> Self {
        HumanController {
            current_action: Action::Wait,
        }
    }

    fn read_input(&self) -> Option<Action> {
        if is_key_down(KeyCode::Right) {
            Some(Action::Right)
        } else if is_key_down(KeyCode::Left) {
            Some(Action::Left)
        } else if is_key_down(KeyCode::Up) {
            Some(Action::Up)
        } else if is_key_down(KeyCode::Down) {
            Some(Action::Down)
        } else {
            None
        }
    }
}