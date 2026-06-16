use std::{env, error::Error, process};
use snake_game::game::{snake_env::SnakeEnv};
use snake_game::agents::{agent::Agent,random_agent::RandomAgent,heuristic_agent::HeuristicAgent,q_learning::QLearningAgent};
use snake_game::controllers::{control::Controller, human_control::HumanController};
use snake_game::render;
use macroquad::prelude::*;

const FILE_PATH: &str = "/home/anders/Documents/MachineLearning/RL/Rust/snake/q_learning";

async fn run_visible(mut controller: Box<dyn Controller>) {
    let mut snake_env = SnakeEnv::new();
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
    let spec = config.controller;
    let mode = config.mode;

    match (mode, spec.load_visible) {
        (RunMode::Visible, None) => {
            let controller = (spec.create_visible)();
            run_visible(controller).await;
            return Ok(());
        }
        (RunMode::Visible, Some(load)) => {
            let loaded_agent = load(FILE_PATH)?;
            run_visible(loaded_agent).await;
            return Ok(());
        }
        (RunMode::Training, _) => {
            let create_agent =
                (spec.create_training).ok_or("Agent does not support training")?;
            let mut agent = create_agent();
            let env = SnakeEnv::new();
            agent.run_training(env);
            agent.save(FILE_PATH)?;
            return Ok(());
        }
        (RunMode::Evaluation, Some(load)) => {
            let mut loaded_agent = load(FILE_PATH)?;
            let env = SnakeEnv::new();
            let score = loaded_agent.evaluate(env);
            println!("Score: {score}");
            return Ok(());
        }
        (RunMode::Evaluation, _) => {
            let env = SnakeEnv::new();
            let create_agent =
                (spec.create_training).ok_or("Agent does not support evaluation")?;
            let mut agent = create_agent();
            let score = agent.evaluate(env);
            println!("Score: {score}");
            return Ok(());
        }
    }
}

fn find_agent(name: &str) -> Result<&'static AgentSpec, &'static str> {
    AGENTS
        .iter()
        .find(|agent| agent.name == name)
        .ok_or("Unknown agent \n")
}

struct Config {
    controller: &'static AgentSpec,
    mode: RunMode,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments \n");
        }
        let controller = find_agent(&args[1])?;
        let mode = RunMode::from_str(&args[2])?;
        Ok(Config { controller, mode })
    }
}

enum RunMode {
    Visible,
    Training,
    Evaluation,
}

impl RunMode {
    fn from_str(s: &str) -> Result<Self, &'static str> {
        match s {
            "visible" => Ok(RunMode::Visible),
            "training" => Ok(RunMode::Training),
            "evaluation" => Ok(RunMode::Evaluation),
            _ => Err("Invalid Mode \n"),
        }
    }
}

struct AgentSpec {
    name: &'static str,
    create_visible: fn() -> Box<dyn Controller>,
    create_training: Option<fn() -> Box<dyn Agent>>,
    load_visible: Option<fn(&str) -> Result<Box<dyn Agent>, Box<dyn std::error::Error>>>,
}

static AGENTS: &[AgentSpec] = &[
    AgentSpec {
        name: "random",
        create_visible: || Box::new(RandomAgent::new()),
        create_training: Some(|| Box::new(RandomAgent::new())),
        load_visible: None,
    },
    AgentSpec {
        name: "heuristic",
        create_visible: || Box::new(HeuristicAgent::new()),
        create_training: Some(|| Box::new(HeuristicAgent::new())),
        load_visible: None,
    },
    AgentSpec {
        name: "human",
        create_visible: || Box::new(HumanController::new()),
        create_training: None,
        load_visible: None,
    },
    AgentSpec {
        name: "q_learning",
        create_visible: || Box::new(QLearningAgent::new()),
        create_training: Some(|| Box::new(QLearningAgent::new())),
        load_visible: Some(|path| {
            let mut agent = QLearningAgent::load(path)?;
            agent.epsilon = 0.0;
            Ok(Box::new(agent))
        }),
    },
];
