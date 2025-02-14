use macroquad::prelude::{draw_texture_ex, DrawTextureParams, Texture2D, Vec2, WHITE};

pub struct Bird {
    pub y: f32,
    pub velocity: f32,
}

impl Bird {
    pub fn update(&mut self, jump_sound: &macroquad::audio::Sound) {
        // Update logic
    }

    pub fn draw(&self, texture: &Texture2D) {
        draw_texture_ex(
            *texture,
            50.0,
            self.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(30.0, 30.0)),
                ..Default::default()
            },
        );
    }
}