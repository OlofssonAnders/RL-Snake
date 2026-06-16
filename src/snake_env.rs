use crate::game::{Action, Observation, Game};

pub struct SnakeEnv {
    pub game: Game,
    fruit_distance: i8,
}

impl SnakeEnv {

    pub fn new() -> Self {
        SnakeEnv {
            game: Game::new(16, 0.1),
            fruit_distance:  0,
        }
    }

    pub fn reset(&mut self) -> Observation {
        self.game = Game::new(16, 0.1);
        self.game.observe()
    }

    pub fn step(&mut self, action: Action) -> (Observation, f32, bool) {
        self.game.act(action);
        self.fruit_distance = self.game.fruit_distance();
        self.game.step_snake();

        let reward = self.compute_reward();
        let done = self.is_done();

        (self.game.observe(), reward, done)
    }

    pub fn compute_reward(&self) -> f32 {
        // Compute the reward based on the current game state, such as positive reward for eating fruit and negative reward for collisions.
        if self.game.get_game_over() {
            -1.0
        } else if self.game.ate_fruit() {
            50.0
        } else if self.game.fruit_distance() < self.fruit_distance {
            10.0
        } else { 
            0.001
        }
    }

    pub fn is_done(&self) -> bool {
        // Check if the game is over, either by collision or other end conditions.
        self.game.get_game_over()
    }

    pub fn observe(&self) -> Observation {
        self.game.observe()
    }
}
