use crate::{agents::control::Controller, game::snake_env::SnakeEnv};

pub trait Agent: Controller {
    fn run_training(&mut self, env: SnakeEnv);

    fn save(&self, path: &str) -> Result<(), Box<dyn std::error::Error>>;

    fn evaluate(&mut self, mut env: SnakeEnv) -> i32 {
        let mut total_score = 0;
        let total_episodes = 5000;
        for episode in 0..total_episodes {
            let mut obs = env.reset();
            loop {
                let action = self.choose_action(&obs);
                let (next_obs, _, done) = env.step(action);
                obs = next_obs;
                if done {
                    total_score += env.game.get_score();
                    break;
                }
            }
            if episode % 1000 == 0 {
                println!("Episode: {episode}");
            }
        }
        return total_score/total_episodes;
    }
}
