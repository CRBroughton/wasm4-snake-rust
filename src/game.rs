
use crate::snake::{Point, Snake};
use crate::wasm4;

pub struct Game {
    snake: Snake,
}

impl Game {
    pub fn new() -> Self {
        Self {
            snake: Snake::new(),
        }
    }

    pub fn update(&self) {
        self.snake.draw();
    }
}