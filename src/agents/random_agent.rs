use crate::game::{Action, Observation};
use crate::controllers::control::{Controller};
use crate::agents::agent::{Agent};
use crate::snake_env::SnakeEnv;
use rand::prelude::*;

pub struct RandomAgent {
    // Placeholder for agent's internal state, e.g., Q-table, neural network, etc.
}

impl Controller for RandomAgent {
    fn choose_action(&mut self, _obs: &Observation) -> Action {
        let mut rng = rand::rng();

        match rng.random_range(0..5) {
            0 => Action::Up,
            1 => Action::Down,
            2 => Action::Left,
            3 => Action::Right,
            _ => Action::Wait,
        }
    }
}

impl Agent for RandomAgent {
    fn run_training(&mut self, env: SnakeEnv) {
        let _ = env;
        // Nothing to train here
    }
    
    fn save(&self, _: &str) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }
}

impl RandomAgent {
    pub fn new() -> Self {
        RandomAgent {
            // Initialize agent's internal state here.
        }
    }
}