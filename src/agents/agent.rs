use crate::{controllers::control::Controller, snake_env::SnakeEnv};

pub trait Agent: Controller {
    fn run_training(&mut self, env: SnakeEnv);

    fn save(&self, path: &str) -> Result<(), Box<dyn std::error::Error>>;

    fn evaluate(&mut self, mut env: SnakeEnv) -> i32 {
        let mut avg_score = 0;
        for episode in 0..30000 {
            let mut obs = env.reset();
            loop {
                let action = self.choose_action(&obs);
                let (next_obs, _, done) = env.step(action);
                obs = next_obs;
                if done {
                    avg_score = (avg_score*episode + env.game.get_score()) / (episode + 1);
                    break;
                }
            }
            if episode % 1000 == 0 {
                println!("Episode: {episode}")
            }
        }
        return avg_score;
    }
}
