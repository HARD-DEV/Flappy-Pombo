use macroquad::audio::{play_sound, PlaySoundParams};
use macroquad::color::YELLOW;
use macroquad::input::{is_key_pressed, KeyCode};
use macroquad::shapes::draw_circle;

const GRAVITY: f32 = 0.5;
const JUMP_STRENGTH: f32 = -8.0;

pub struct Bird {
    pub y: f32,
    pub velocity: f32,
}

impl Bird {
    pub fn update(&mut self, jump_sound: &macroquad::audio::Sound) {
        self.velocity += GRAVITY;
        self.y += self.velocity;

        if is_key_pressed(KeyCode::Space) {
            self.velocity = JUMP_STRENGTH;
            play_sound(
                jump_sound,
                PlaySoundParams {
                    looped: false,
                    volume: 1.0,
                },
            );
        }

        if self.y < 0.0 {
            self.y = 0.0;
            self.velocity = 0.0;
        }
    }

    pub fn draw(&self) {
        draw_circle(50.0, self.y, 15.0, YELLOW);
    }
}
