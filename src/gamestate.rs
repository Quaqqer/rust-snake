use crate::{snake::Snake, vec2::Point2D};
use rand::random;

pub struct GameState {
    pub width: u32,
    pub height: u32,

    pub snake: crate::snake::Snake,
    pub fruit: Option<Point2D>,
    score: i32,
}

pub enum TickResult {
    Continue,
    Exit(i32), // score
}

impl GameState {
    pub fn tick(&mut self) -> TickResult {
        self.snake.tick();

        if self.fruit.is_some() && self.snake.pos == self.fruit.unwrap() {
            self.fruit = Some(self.gen_fruit());
            self.snake.queued_grow += 1;
            self.score += 1;
        }

        if !(0 <= self.snake.pos.x && self.snake.pos.x < self.width as i32 &&
             0 <= self.snake.pos.y && self.snake.pos.y < self.height as i32) {
            return TickResult::Exit(self.score);
        }

        if self.snake.collides_tail() {
            return TickResult::Exit(self.score);
        }

        TickResult::Continue
    }

    pub fn gen_fruit(&self) -> Point2D {
        loop {
            let fx = (random::<f32>() * self.width  as f32) as i32;
            let fy = (random::<f32>() * self.height as f32) as i32;

            let p = Point2D::new(fx, fy);

            if !self.snake.contains_point(p) {
                return p;
            }
        }
    }

    pub fn new(width: u32, height: u32) -> GameState {
        let mut snake = Snake::new(width, height, 1);
        snake.queued_grow = 4;
        let mut gs = GameState { width, height, snake, fruit: None, score: 0 };
        gs.fruit = Some(gs.gen_fruit());

        gs
    }
}
