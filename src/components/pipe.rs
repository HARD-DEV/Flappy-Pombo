use macroquad::prelude::{draw_rectangle, GREEN};
use macroquad::prelude::{screen_height, screen_width};
use crate::PIPE_WIDTH;

let mut pipes: Vec<Pipe> = vec![Pipe { ... }];

pub struct Pipe {
    pub x: f32,
    pub height: f32,
}

impl Pipe {
    pub fn update(&mut self, speed: f32) {
        self.x -= speed;
    }

    pub fn draw(&self, gap: f32) {
        draw_rectangle(self.x, 0.0, PIPE_WIDTH, self.height, GREEN);
        draw_rectangle(
            self.x,
            self.height + gap,
            PIPE_WIDTH,
            screen_height() - self.height - gap,
            GREEN,
        );
    }
}
