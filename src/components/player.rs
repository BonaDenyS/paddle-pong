use macroquad::prelude::*;

// Input player names
pub async fn get_player_name(side: &str) -> String {
    let mut name = String::new();
    loop {
        clear_background(BLACK);
        draw_text(&format!("Enter {} player name: {}", side, name), 50.0, 350.0, 30.0, WHITE);
        next_frame().await;
        if let Some(c) = get_char_pressed() {
            match c {
                '\r' | '\n' if !name.is_empty() => break,
                '\x08' => { name.pop(); }
                _ => name.push(c),
            }
        }
    }
    name
}