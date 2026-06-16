use crate::game::{Action, Observation};
use rand::prelude::*;
use std::collections::LinkedList;

pub type Point = (i8, i8);

struct Snake {
    head: Point,
    body: LinkedList<Point>,
    new_dir: Action,
    prev_dir: Action,
}

pub struct Game {
    snake: Snake,
    fruit: Point,
    score: i32,
    speed: f32,
    game_over: bool,
    squares: i8,
}

impl Game {
    pub fn new(squares: i8, speed: f32) -> Self {
        let starting_pos = random_position(squares);
        let starting_pos_fruit = random_position(squares);
        Game {
            snake: Snake {
                head: starting_pos,
                body: LinkedList::new(),
                new_dir: Action::Right,
                prev_dir: Action::Wait,
            },
            fruit: starting_pos_fruit,
            score: 0,
            speed: speed,
            game_over: false,
            squares: squares,
        }
    }

    pub fn get_game_over(&self) -> bool {
        self.game_over
    }

    pub fn ate_fruit(&self) -> bool {
        self.snake.head == self.fruit
    }

    pub fn fruit_distance(&self) -> i8 {
        let dist_x = (self.snake.head.0 - self.fruit.0).abs();
        let dist_y = (self.snake.head.1 - self.fruit.1).abs();
        dist_x + dist_y
    }

    pub fn get_score(&self) -> i32 {
        return self.score;
    }

    pub fn observe(&self) -> Observation {
        // Create an observation of the current game state, including the snake's position, fruit position, and other relevant information.
        Observation {
            snake_head: self.snake.head,
            snake_body: self.snake.body.clone(),
            fruit: self.fruit,
            score: self.score,
            game_over: self.game_over,
            squares: self.squares,
            speed: self.speed,
            prev_dir: self.snake.prev_dir,
        }
    }

    pub fn act(&mut self, action: Action) {
        // Update the snake's direction based on the input action, ensuring it cannot reverse directly.
        match action {
            Action::Up => {
                if self.snake.prev_dir != Action::Down {
                    self.snake.new_dir = Action::Up;
                }
            }
            Action::Down => {
                if self.snake.prev_dir != Action::Up {
                    self.snake.new_dir = Action::Down;
                }
            }
            Action::Right => {
                if self.snake.prev_dir != Action::Left {
                    self.snake.new_dir = Action::Right;
                }
            }
            Action::Left => {
                if self.snake.prev_dir != Action::Right {
                    self.snake.new_dir = Action::Left;
                }
            }
            Action::Wait => {
                // Do nothing, keep the current direction.
            }
        }
    }

    pub fn step_snake(&mut self) {
        // Update the game state by moving the snake, checking for collisions, and updating the score and game over status.
        if !self.game_over {
            self.move_snake();
            self.snake.prev_dir = self.snake.new_dir.clone();
            if self.snake.head == self.fruit {
                self.fruit = self.random_fruit(self.squares);
                self.score += 100;
                self.speed *= 1.0 //0.9;
            } else {
                self.snake.body.pop_back();
            }
            if self.border_collision(&self.snake, self.squares) {
                self.game_over = true;
            }
            if self.self_collision(&self.snake) {
                self.game_over = true;
            }
        }
    }

    fn random_fruit(&self, squares: i8) -> (i8, i8) {
        let mut rng = rand::rng();
        loop {
            let pos = (rng.random_range(0..squares), rng.random_range(0..squares));

            if !self.snake.body.contains(&pos) && !(self.snake.head == pos) {
                return pos;
            }
        }
    }

    fn self_collision(&self, snake: &Snake) -> bool {
        for (x, y) in &snake.body {
            if *x == snake.head.0 && *y == snake.head.1 {
                return true;
            }
        }
        false
    }

    fn border_collision(&self, snake: &Snake, squares: i8) -> bool {
        snake.head.0 < 0 || snake.head.1 < 0 || snake.head.0 >= squares || snake.head.1 >= squares
    }

    fn move_snake(&mut self) {
        self.snake.body.push_front(self.snake.head);
        self.snake.head = (
            self.snake.head.0 + self.snake.new_dir.to_point().0,
            self.snake.head.1 + self.snake.new_dir.to_point().1,
        );
    }
}

fn random_position(squares: i8) -> (i8, i8) {
    let mut rng = rand::rng();
    (rng.random_range(0..squares), rng.random_range(0..squares))
}
