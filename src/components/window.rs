use macroquad::window::Conf;
use macroquad::window::miniquad::conf::Icon;

// Window configuration with icon
pub fn window_conf() -> Conf {
    Conf {
        window_title: "Paddle Pong".to_string(),
        window_width: 900,
        window_height: 700,
        icon: load_icon(),
        ..Default::default()
    }
}

// Load icon from PNG file
fn load_icon() -> Option<Icon> {
    let img = image::open("assets/racket.png").ok()?;

    // Small: 32x32
    let small = img.resize_exact(32, 32, image::imageops::FilterType::Nearest)
        .to_rgba8()
        .into_raw();
    if small.len() != 1024 { return None; }
    let mut small_arr = [0u8; 1024];
    small_arr.copy_from_slice(&small);

    // Medium: 64x64
    let medium = img.resize_exact(64, 64, image::imageops::FilterType::Nearest)
        .to_rgba8()
        .into_raw();
    if medium.len() != 4096 { return None; }
    let mut medium_arr = [0u8; 4096];
    medium_arr.copy_from_slice(&medium);

    // Big: 128x128
    let big = img.resize_exact(128, 128, image::imageops::FilterType::Nearest)
        .to_rgba8()
        .into_raw();
    if big.len() != 16384 { return None; }
    let mut big_arr = [0u8; 16384];
    big_arr.copy_from_slice(&big);

    Some(Icon {
        small: small_arr,
        medium: medium_arr,
        big: big_arr,
    })
}
