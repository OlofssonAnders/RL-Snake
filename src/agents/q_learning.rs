use crate::agents::agent::Agent;
use crate::controllers::control::Controller;
use crate::game::game_logic::Point;
use crate::game::{Action, Observation};
use crate::game::snake_env::SnakeEnv;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::LinkedList;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct QLearningAgent {
    q: HashMap<(StateKey, Action), f32>,
    alpha: f32,
    gamma: f32,
    pub epsilon: f32,
}

#[derive(Serialize, Deserialize)]
pub struct QEntry {
    pub state: StateKey,
    pub action: Action,
    pub value: f32,
}

#[derive(Serialize, Deserialize)]
pub struct QLearningSave {
    pub q: Vec<QEntry>,
    pub alpha: f32,
    pub gamma: f32,
    pub epsilon: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StateKey {
    danger_up: bool,
    danger_down: bool,
    danger_left: bool,
    danger_right: bool,
    food_dx: i8, // -1, 0, 1
    food_dy: i8, // -1, 0, 1
}

impl From<&Observation> for StateKey {
    fn from(obs: &Observation) -> Self {
        let head = obs.snake_head;

        let up = (head.0, head.1 - 1);
        let down = (head.0, head.1 + 1);
        let left = (head.0 - 1, head.1);
        let right = (head.0 + 1, head.1);

        let food_dx = signum_i8(obs.fruit.0 - head.0);
        let food_dy = signum_i8(obs.fruit.1 - head.1);

        Self {
            danger_up: is_collision(up, &obs.snake_body, obs.squares),
            danger_down: is_collision(down, &obs.snake_body, obs.squares),
            danger_left: is_collision(left, &obs.snake_body, obs.squares),
            danger_right: is_collision(right, &obs.snake_body, obs.squares),
            food_dx,
            food_dy,
        }
    }
}

fn is_collision(pos: Point, body: &LinkedList<Point>, squares: i8) -> bool {
    pos.0 < 0 || pos.0 >= squares || pos.1 < 0 || pos.1 >= squares || body.contains(&pos)
}

fn signum_i8(value: i8) -> i8 {
    if value > 0 {
        1
    } else if value < 0 {
        -1
    } else {
        0
    }
}

const ACTIONS: [Action; 5] = [
    Action::Up,
    Action::Down,
    Action::Left,
    Action::Right,
    Action::Wait,
];

impl Controller for QLearningAgent {
    fn choose_action(&mut self, obs: &Observation) -> Action {
        let mut rng = rand::rng();
        let state = StateKey::from(obs);

        if rng.random::<f32>() < self.epsilon {
            return ACTIONS[rng.random_range(0..ACTIONS.len())];
        }

        ACTIONS
            .iter()
            .copied()
            .max_by(|a, b| {
                let qa = self.q.get(&(state, *a)).unwrap_or(&0.0);
                let qb = self.q.get(&(state, *b)).unwrap_or(&0.0);
                qa.partial_cmp(qb).unwrap()
            })
            .unwrap()
    }
}

impl Agent for QLearningAgent {
    fn run_training(&mut self, mut env: SnakeEnv) {
        for episode in 0..30000 {
            let mut obs = env.reset();
            loop {
                let action = self.choose_action(&obs);
                let (next_obs, reward, done) = env.step(action);
                self.learn(&obs, action, reward, &next_obs, done);
                obs = next_obs;
                if done {
                    break;
                }
            }
            if episode % 1000 == 0 {
                println!("Episode: {episode}")
            }
            self.epsilon *= 0.999; // reduce exploration slowly
        }
    } 

    fn save(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let entries: Vec<QEntry> = self
            .q
            .iter()
            .map(|((state, action), value)| QEntry {
                state: *state,
                action: *action,
                value: *value,
            })
            .collect();

        let save = QLearningSave {
            q: entries,
            alpha: self.alpha,
            gamma: self.gamma,
            epsilon: self.epsilon,
        };

        let file = File::create(path)?;
        serde_json::to_writer_pretty(BufWriter::new(file), &save)?;
        Ok(())
    }
}

impl QLearningAgent {
    pub fn new() -> Self {
        QLearningAgent {
            q: HashMap::new(),
            alpha: 0.1,
            gamma: 0.95,
            epsilon: 1.0,
        }
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let save: QLearningSave = serde_json::from_reader(BufReader::new(file))?;

        let q = save
            .q
            .into_iter()
            .map(|entry| ((entry.state, entry.action), entry.value))
            .collect();

        Ok(Self {
            q,
            alpha: save.alpha,
            gamma: save.gamma,
            epsilon: save.epsilon,
        })
    }

    pub fn learn(
        &mut self,
        obs: &Observation,
        action: Action,
        reward: f32,
        next_obs: &Observation,
        done: bool,
    ) {
        let state = StateKey::from(obs);
        let next_state = StateKey::from(next_obs);

        let old_q = *self.q.get(&(state, action)).unwrap_or(&0.0);

        let next_max = if done {
            0.0
        } else {
            ACTIONS
                .iter()
                .map(|a| *self.q.get(&(next_state, *a)).unwrap_or(&0.0))
                .fold(f32::NEG_INFINITY, f32::max)
        };

        let target = reward + self.gamma * next_max;
        let new_q = old_q + self.alpha * (target - old_q);

        self.q.insert((state, action), new_q);
    }
}
