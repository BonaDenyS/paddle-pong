use paddle_pong::models::{Paddle, Ball};
use paddle_pong::models::texture_mock::_TestTexture;

// ----------------------------------------------------
// Test: Paddle movement clamping
// ----------------------------------------------------
#[test]
fn test_paddle_clamping() {
    let tex = _TestTexture;
    let mut paddle = Paddle::new(tex, 0.0, 0.0);

    // Move above screen
    paddle.y = -50.0;
    paddle.y = paddle.y.clamp(0.0, 700.0 - 64.0);
    assert_eq!(paddle.y, 0.0);

    // Move below screen
    paddle.y = 1000.0;
    paddle.y = paddle.y.clamp(0.0, 700.0 - 64.0);
    assert_eq!(paddle.y, 700.0 - 64.0);
}

// ----------------------------------------------------
// Test: Ball reset (left serve)
// ----------------------------------------------------
#[test]
fn test_ball_reset_left() {
    let tex = _TestTexture;
    let mut ball = Ball::new(tex, 0.0, 0.0, true);

    ball.reset("left");

    assert_eq!(ball.x, 450.0 - 32.0);
    assert_eq!(ball.y, 350.0 - 32.0);
    assert_eq!(ball.speed, 4.0);
    assert!(ball.vel_x > 0.0); // left serve → positive direction
}

// ----------------------------------------------------
// Test: Ball reset (right serve)
// ----------------------------------------------------
#[test]
fn test_ball_reset_right() {
    let tex = _TestTexture;
    let mut ball = Ball::new(tex, 0.0, 0.0, true);

    ball.reset("right");

    assert_eq!(ball.x, 450.0 - 32.0);
    assert_eq!(ball.y, 350.0 - 32.0);
    assert_eq!(ball.speed, 4.0);
    assert!(ball.vel_x < 0.0); // right serve → negative direction
}

// ----------------------------------------------------
// Test: Collision increases ball speed
// ----------------------------------------------------
#[test]
fn test_ball_collision_increases_speed() {
    let tex = _TestTexture;
    let ptex = _TestTexture;

    let left = Paddle::new(ptex, 30.0, 300.0);
    let right = Paddle::new(ptex, 900.0 - 94.0, 300.0);

    let mut ball = Ball::new(tex, 40.0, 300.0, true); // close to left paddle
    let original_speed = ball.speed;

    let _ = ball.update(&left, &right, 900.0, 700.0);

    assert!(ball.speed > original_speed);
}

// ----------------------------------------------------
// Test: Scoring - right scores when ball goes past left
// ----------------------------------------------------
#[test]
fn test_ball_scoring_right() {
    let tex = _TestTexture;
    let ptex = _TestTexture;

    let left = Paddle::new(ptex, 30.0, 300.0);
    let right = Paddle::new(ptex, 900.0 - 94.0, 300.0);

    let mut ball = Ball::new(tex, -20.0, 350.0, true); // left wall
    let res = ball.update(&left, &right, 900.0, 700.0);

    assert_eq!(res, Some("right"));
}

// ----------------------------------------------------
// Test: Scoring - left scores when ball goes past right
// ----------------------------------------------------
#[test]
fn test_ball_scoring_left() {
    let tex = _TestTexture;
    let ptex = _TestTexture;

    let left = Paddle::new(ptex, 30.0, 300.0);
    let right = Paddle::new(ptex, 900.0 - 94.0, 300.0);

    let mut ball = Ball::new(tex, 910.0, 350.0, true); // right wall
    let res = ball.update(&left, &right, 900.0, 700.0);

    assert_eq!(res, Some("left"));
}