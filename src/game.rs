
use fastrand::Rng;
use crate::{snake::Snake, snake::Point};
use crate::wasm4;

pub struct Game {
    rng: Rng,
    snake: Snake,
    frame_count: u32,
    prev_gamepad: u8,
    fruit: Point,
}

impl Game {
    pub fn new() -> Self {
        let rng = Rng::with_seed(235);
        Self {
            snake: Snake::new(),
            frame_count: 0,
            prev_gamepad: 0,
            fruit: Point {
                x: rng.i32(0..20),
                y: rng.i32(0..20)
            },
        }
    }

    pub fn update(&mut self) {
        self.frame_count += 1;

        self.input();
        
        if self.frame_count % 15 == 0 {
            self.snake.update();
        }
        self.snake.draw();
    }
    
    pub fn input(&mut self) {
        let gamepad = unsafe { *wasm4::GAMEPAD1 };
        let just_pressed = gamepad & (gamepad ^ self.prev_gamepad);

        if just_pressed & wasm4::BUTTON_LEFT != 0 {
            self.snake.left();
        }
        if just_pressed & wasm4::BUTTON_RIGHT != 0 {
            self.snake.right();
        }
        if just_pressed & wasm4::BUTTON_UP != 0 {
            self.snake.up();
        }
        if just_pressed & wasm4::BUTTON_DOWN != 0 {
            self.snake.down();
        }
        self.prev_gamepad = gamepad;
    }
}