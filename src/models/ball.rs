use macroquad::prelude::{Texture2D, draw_texture_ex, DrawTextureParams, WHITE, Vec2};
use rand::{Rng, rng};
use crate::models::paddle::Paddle;

// Ball structure
pub struct Ball<T> {
    pub texture: T,
    pub x: f32,
    pub y: f32,
    pub vel_x: f32,
    pub vel_y: f32,
    pub speed: f32,
}

impl Ball<Texture2D> {
    pub fn draw(&self) {
        draw_texture_ex(
            self.texture,
            self.x,
            self.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(64.0, 64.0)),
                ..Default::default()
            },
        );
    }
}

impl <T: Copy> Ball<T> {
    pub fn new(texture: T, x: f32, y: f32, start_from_left: bool) -> Self {
        let mut rng = rng();
        let vel_x = if start_from_left { 1.0 } else { -1.0 } * 4.0;
        let vel_y = 4.0 * if rng.random_bool(0.5) { 1.0 } else { -1.0 };
        Self { texture, x, y, vel_x, vel_y, speed: 4.0 }
    }

    pub fn update(&mut self, left: &Paddle<T>, right: &Paddle<T>, screen_width: f32, screen_height: f32) -> Option<&'static str> {
        self.x += self.vel_x;
        self.y += self.vel_y;

        if self.y <= 0.0 || self.y + 64.0 >= screen_height { self.vel_y = -self.vel_y; }

        // Paddle collisions
        if self.x <= left.x + 64.0 && self.y + 64.0 >= left.y && self.y <= left.y + 64.0 {
            self.vel_x = self.vel_x.abs();
            self.speed += 1.0;
            self.vel_x = self.vel_x.signum() * self.speed;
            self.vel_y = self.vel_y.signum() * self.speed;
        }

        if self.x + 64.0 >= right.x && self.y + 64.0 >= right.y && self.y <= right.y + 64.0 {
            self.vel_x = -self.vel_x.abs();
            self.speed += 1.0;
            self.vel_x = self.vel_x.signum() * self.speed;
            self.vel_y = self.vel_y.signum() * self.speed;
        }

        // Score
        if self.x < 0.0 { self.reset("right"); return Some("right"); }
        if self.x > screen_width { self.reset("left"); return Some("left"); }

        None
    }

    pub fn reset(&mut self, start_from: &str) {
        self.x = 450.0 - 32.0;
        self.y = 350.0 - 32.0;
        self.speed = 4.0;
        let mut rng = rng();
        self.vel_x = if start_from == "left" { 1.0 } else { -1.0 } * self.speed;
        self.vel_y = self.speed * if rng.random_bool(0.5) { 1.0 } else { -1.0 };
    }
}