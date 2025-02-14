use macroquad::prelude::{draw_texture_ex, Color, vec2, Texture2D};
use macroquad::texture::DrawTextureParams;
use rand::{thread_rng, Rng};

const CLOUD_SPEED_RANGE: (f32, f32) = (0.5, 1.5);
const CLOUD_SPAWN_Y_RANGE: (f32, f32) = (50.0, 300.0);
const MIN_CLOUD_SCALE: f32 = 0.2;
const MAX_CLOUD_SCALE: f32 = 0.4;

pub struct Cloud {
    pub texture: Texture2D,
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub scale: f32,
    pub alpha: f32,
}

impl Cloud {
    pub async fn new(textures: &[Texture2D]) -> Self {
        let mut rng = thread_rng();
        let texture = textures[rng.gen_range(0..textures.len())].clone();
        
        Self {
            texture,
            x: rng.gen_range(screen_width()..screen_width() + 500.0),
            y: rng.gen_range(CLOUD_SPAWN_Y_RANGE.0..CLOUD_SPAWN_Y_RANGE.1),
            speed: rng.gen_range(CLOUD_SPEED_RANGE.0..CLOUD_SPEED_RANGE.1),
            scale: rng.gen_range(MIN_CLOUD_SCALE..MAX_CLOUD_SCALE),
            alpha: 0.0,
        }
    }

    pub fn update(&mut self) {
        self.x -= self.speed;
        self.alpha = (self.alpha + 0.02).clamp(0.0, 1.0);

        if self.x < -self.texture.width() * self.scale {
            let mut rng = thread_rng();
            self.x = screen_width() + rng.gen_range(100.0..400.0);
            self.y = rng.gen_range(CLOUD_SPAWN_Y_RANGE.0..CLOUD_SPAWN_Y_RANGE.1);
            self.speed = rng.gen_range(CLOUD_SPEED_RANGE.0..CLOUD_SPEED_RANGE.1);
            self.scale = rng.gen_range(MIN_CLOUD_SCALE..MAX_CLOUD_SCALE);
            self.alpha = 0.0;
        }
    }

    pub fn draw(&self) {
        draw_texture_ex(
            &self.texture,
            self.x,
            self.y,
            Color::from_rgba(255, 255, 255, (self.alpha * 255.0) as u8),
            vec2(
                self.texture.width() * self.scale,
                self.texture.height() * self.scale,
            ),
        );
    }
}
