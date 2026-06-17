use macroquad::prelude::*;
use snake_game::agents::{
    agent::Agent, heuristic_agent::HeuristicAgent, q_learning::QLearningAgent,
    random_agent::RandomAgent,
};
use snake_game::agents::{control::Controller, human_control::HumanController};
use snake_game::game::snake_env::SnakeEnv;
use snake_game::render;
use std::{env, error::Error, process};

const SAVE_PATH: &'static str = "/home/anders/Documents/MachineLearning/RL/Rust/snake/";

async fn run_visible(mut snake_env: SnakeEnv, mut controller: Box<dyn Controller>) {
    let mut accumulator = 0.0;
    let mut step_time = 0.1; // seconds per snake move

    let mut obs = snake_env.observe();

    loop {
        accumulator += get_frame_time();

        let action = controller.choose_action(&obs);
        if accumulator >= step_time {
            let (new_obs, _reward, _done) = snake_env.step(action);
            obs = new_obs;
            accumulator -= step_time;
            step_time = obs.speed;
        }

        if snake_env.is_done() {
            obs = snake_env.reset();
            accumulator = 0.0;
            step_time = 0.1;
        }

        render::draw_game(&snake_env.game);
        next_frame().await;
    }
}

#[macroquad::main("Snake RL")]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        print!("Problem parsing arguments : {err} \n");
        process::exit(1);
    });
    if let Err(e) = run(config).await {
        println!("Application error : {e} \n");
        process::exit(1);
    }
}

async fn run(config: Config) -> Result<(), Box<dyn Error>> {
    
    let snake_env = SnakeEnv::new();

    match config {
        Config::Human(controller) => {
            run_visible(snake_env, Box::new(controller)).await;
        }
        Config::Visible(spec) => {
            let mut agent = (spec.create)();
            let path = String::from(SAVE_PATH) + spec.name;
            if let Some(load) = spec.load {
                agent = load(&path)?;
            } 
            run_visible(snake_env, agent).await;
        }
        Config::Training(spec) => {
            let mut agent = (spec.create)();
            let path = String::from(SAVE_PATH) + spec.name;
            agent.run_training(snake_env);
            agent.save(&path)?;
        }
        Config::Evaluation(spec) => {
            let mut agent = (spec.create)();
            let path = String::from(SAVE_PATH) + spec.name;
            if let Some(load) = spec.load {
                agent = load(&path)?;
            }
            let score = agent.evaluate(snake_env);
            println!("Score: {score}");
        }
    }
    return Ok(());
}

fn find_agent(name: &str) -> Result<&'static AgentSpec, &'static str> {
    AGENTS
        .iter()
        .find(|agent| agent.name == name)
        .ok_or("Unknown agent \n")
}

enum Config {
    Human(HumanController),
    Visible(&'static AgentSpec),
    Training(&'static AgentSpec),
    Evaluation(&'static AgentSpec),
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments \n");
        }
        return Config::from_str(&args[1], &args[2]);
    }

    fn from_str(s: &str, g: &str) -> Result<Self, &'static str> {
        match s {
            "human" => Ok(Config::Human(HumanController::new())),
            "visible" => Ok(Config::Visible(find_agent(g)?)),
            "training" => Ok(Config::Training(find_agent(g)?)),
            "evaluation" => Ok(Config::Evaluation(find_agent(g)?)),
            _ => Err("Invalid Mode \n"),
        }
    }
}

struct AgentSpec {
    name: &'static str,
    create: fn() -> Box<dyn Agent>,
    load: Option<fn(&str) -> Result<Box<dyn Agent>, Box<dyn std::error::Error>>>,
}

static AGENTS: &[AgentSpec] = &[
    AgentSpec {
        name: "random",
        create: || Box::new(RandomAgent::new()),
        load: None,
    },
    AgentSpec {
        name: "heuristic",
        create: || Box::new(HeuristicAgent::new()),
        load: None,
    },
    AgentSpec {
        name: "q_learning",
        create: || Box::new(QLearningAgent::new()),
        load: Some(|path: &str| {
            let mut agent = QLearningAgent::load(path)?;
            agent.epsilon = 0.0;
            Ok(Box::new(agent))
        }),
    },
];
