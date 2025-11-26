use macroquad::prelude::*;

// Paddle structure
pub struct Paddle<T> {
    pub texture: T,
    pub x: f32,
    pub y: f32,
    pub speed: f32,
}

impl <T: Copy> Paddle<T> {
    pub fn new(texture: T, x: f32, y: f32) -> Self {
        Self { texture, x, y, speed: 6.0 }
    }

    pub fn update(&mut self, up: KeyCode, down: KeyCode, screen_height: f32) {
        if is_key_down(up) { self.y -= self.speed; }
        if is_key_down(down) { self.y += self.speed; }
        self.y = self.y.clamp(0.0, screen_height - 64.0);
    }
}

impl Paddle<Texture2D> {
    pub fn draw(&self, flip_x: bool) {
        draw_texture_ex(
            self.texture,
            self.x,
            self.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(64.0, 64.0)),
                flip_x,
                ..Default::default()
            },
        );
    }
}