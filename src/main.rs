use macroquad::prelude::{
    Texture2D, 
    load_texture,
    WHITE, BLACK, 
    clear_background, 
    draw_text, 
    next_frame, 
    KeyCode, 
    FilterMode
};

mod models;
use models::{Paddle, Ball};

mod components;
use components::{get_player_name, window_conf};


#[macroquad::main(window_conf)]
async fn main() {
    let left_name = get_player_name("Left").await;
    let right_name = get_player_name("Right").await;

    let paddle_texture: Texture2D = load_texture("assets/racket.png").await.unwrap();
    paddle_texture.set_filter(FilterMode::Nearest);
    let ball_texture: Texture2D = load_texture("assets/ball.png").await.unwrap();
    ball_texture.set_filter(FilterMode::Nearest);

    let mut left = Paddle::new(paddle_texture, 30.0, 350.0 - 32.0);
    let mut right = Paddle::new(paddle_texture, 900.0 - 94.0, 350.0 - 32.0);
    let mut ball = Ball::new(ball_texture, 450.0 - 32.0, 350.0 - 32.0, true);

    let mut left_score = 0;
    let mut right_score = 0;

    loop {
        clear_background(BLACK);

        left.update(KeyCode::W, KeyCode::S, 700.0);
        right.update(KeyCode::Up, KeyCode::Down, 700.0);

        if let Some(winner) = ball.update(&left, &right, 900.0, 700.0) {
            if winner == "left" { left_score += 1; } else { right_score += 1; }
        }

        left.draw(false);
        right.draw(true);
        ball.draw();

        draw_text(&format!("{}: {}", left_name, left_score), 50.0, 50.0, 30.0, WHITE);
        draw_text(&format!("{}: {}", right_name, right_score), 700.0, 50.0, 30.0, WHITE);

        next_frame().await;
    }
}
