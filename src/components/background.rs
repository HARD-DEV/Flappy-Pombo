use macroquad::prelude::{draw_texture_ex, screen_width, screen_height, WHITE, Texture2D, vec2, DrawTextureParams};

pub struct Background {
    texture: Texture2D,
    offset: f32,
}

impl Background {
    pub fn new(texture: Texture2D) -> Self {
        Self { texture, offset: 0.0 }
    }

    pub fn update(&mut self, speed: f32) {
        self.offset += speed;
        if self.offset >= self.texture.width() {
            self.offset -= self.texture.width();
        }
    }

    pub fn draw(&self) {
        let screen_width = screen_width();
        let screen_height = screen_height();

        let repeat_times = (screen_width / self.texture.width()).ceil() as i32 + 1;

        for i in 0..repeat_times {
            let x_pos = (i as f32 * self.texture.width()) - self.offset;
            draw_texture_ex(
                &self.texture,
                x_pos,
                0.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(self.texture.width(), screen_height)),
                    ..Default::default()
                },
            );
        }
    }
}
