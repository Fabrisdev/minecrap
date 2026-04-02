use macroquad::prelude::*;

#[macroquad::main("Minecrap")]
async fn main() {
    set_fullscreen(true);
    loop {
        clear_background(BLACK);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);

        draw_text("Hello, Macroquad!", 20.0, 20.0, 30.0, DARKGRAY);
        if (is_key_pressed(KeyCode::Escape)) {
            break;
        }
        next_frame().await
    }
}
