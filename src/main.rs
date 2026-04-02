use macroquad::prelude::*;

const SPEED: f32 = 20.0;

#[macroquad::main("Minecrap")]
async fn main() {
    set_fullscreen(true);
    let mut camera = Camera3D {
        ..Default::default()
    };
    let position = Vec3::default();
    loop {
        set_camera(&camera);
        clear_background(BLACK);
        draw_cube_wires(position, vec3(10.0, 10.0, 10.0), WHITE);
        let delta = get_frame_time();
        if is_key_down(KeyCode::S) {
            camera.position.x -= SPEED * delta;
        }
        if is_key_down(KeyCode::W) {
            camera.position.x += SPEED * delta
        }

        let fps_counter = format!("{} FPS", get_fps());
        draw_text(&fps_counter, 0.0, 25.0, 50.0, WHITE);
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        next_frame().await
    }
}
