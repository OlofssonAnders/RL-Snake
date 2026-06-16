use crate::game::{Action, Observation};


pub trait Controller {
    fn choose_action(&mut self, obs: &Observation) -> Action;
}

