use crate::game::{Action, Observation};
use crate::controllers::Controller;
use crate::agents::agent::Agent;
use crate::snake_env::SnakeEnv;

pub struct HeuristicAgent {
    // Placeholder for agent's internal state, e.g., Q-table, neural network, etc.
}

impl Controller for HeuristicAgent {
    fn choose_action(&mut self, obs: &Observation) -> Action {
        
        let head = obs.snake_head;
        let fruit = obs.fruit;
        let prev_dir: Action = obs.prev_dir;
        
        if head.1 == fruit.1 && head.0 < fruit.0 && prev_dir == Action::Left {
            return Action::Up
        } else if head.1 == fruit.1 && head.0 > fruit.0 && prev_dir == Action::Right {
            return Action::Up
        } else if head.0 == fruit.0 && head.1 < fruit.1 && prev_dir == Action::Up {
            return Action::Left
        } else if head.0 == fruit.0 && head.1 > fruit.1 && prev_dir == Action::Down {
            return Action::Left
        } else if head.1 > fruit.1 && prev_dir != Action::Down {
            return Action::Up
        } else if head.1 < fruit.1 && prev_dir != Action::Up {
            return Action::Down
        } else if head.0 > fruit.0 && prev_dir != Action::Right {
            return Action::Left 
        } else if head.0 < fruit.0 && prev_dir != Action::Left {
            return Action::Right
        } else {
            return Action::Wait
        }
    }
}

impl Agent for HeuristicAgent {
    fn run_training(&mut self, env: SnakeEnv) {
        let _ = env;
        // Implement the training loop for the agent here.
        // This would involve interacting with the environment, updating the agent's internal state based on rewards, etc.
    }

    fn save(&self, _: &str) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }
}

impl HeuristicAgent {
    pub fn new() -> Self {
        HeuristicAgent {
            // Initialize agent's internal state here.
        }
    }
}