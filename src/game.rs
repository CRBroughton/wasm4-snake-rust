
use fastrand::Rng;
use crate::{snake::Snake, snake::Point};
use crate::wasm4::{self, TONE_PULSE1};
use crate::palette::set_draw_colour;

const FRUIT_SPRITE: [u8; 16] = [ 0x00,0xa0,0x02,0x00,0x0e,0xf0,0x36,0x5c,0xd6,0x57,0xd5,0x57,0x35,0x5c,0x0f,0xf0 ];

pub struct Game {
    rng: Rng,
    snake: Snake,
    score: u32,
    frame_count: u32,
    current_speed: u32,
    prev_gamepad: u8,
    fruit: Point,
}

impl Game {
    pub fn new() -> Self {
        let rng = Rng::with_seed(235);
        Self {
            snake: Snake::new(),
            score: 0,
            frame_count: 0,
            current_speed: 15,
            prev_gamepad: 0,
            fruit: Point {
                x: rng.i32(0..20),
                y: rng.i32(0..20)
            },
            rng,
        }
    }

    pub fn update(&mut self) {
        self.frame_count += 1;

        self.input();

        let str = format!("Score: {:}", self.score);

        wasm4::text(str, 1, 1);
        
        if self.frame_count % self.current_speed == 0 {
            let dropped_pos = self.snake.update();

            if self.snake.is_dead() {
                self.current_speed = 15;
                self.score = 0;
                self.snake = Snake::new();
                self.fruit = Point {
                    x: self.rng.i32(0..20),
                    y: self.rng.i32(0..20),
                };
            }

            if self.snake.body[0] == self.fruit {
                if let Some(last_pos) = dropped_pos {
                    self.snake.body.push(last_pos);
                    wasm4::tone(650, 1, 50, TONE_PULSE1);
                    self.score += 1;

                    if self.current_speed > 5 {
                        self.current_speed -= 1
                    }
                }

                self.fruit.x = self.rng.i32(0..20);
                self.fruit.y = self.rng.i32(0..20);
            }
        }
        self.snake.draw();
        
        set_draw_colour(0x4320);
        wasm4::blit(
            &FRUIT_SPRITE,
            self.fruit.x * 8,
            self.fruit.y * 8,
            8,
            8,
            wasm4::BLIT_2BPP,
        );

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