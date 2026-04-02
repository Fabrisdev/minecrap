use std::f32::consts::PI;

use macroquad::prelude::*;

const SPEED: f32 = 4.0;

#[macroquad::main("Minecrap")]
async fn main() {
    set_fullscreen(true);
    show_mouse(false);
    set_cursor_grab(true);

    let mut camera = Camera3D {
        up: vec3(0.0, 1.0, 0.0),
        position: Vec3::ZERO,
        ..Default::default()
    };

    let atlas_texture = Texture2D::from_file_with_format(include_bytes!("assets/atlas.png"), None);
    atlas_texture.set_filter(FilterMode::Nearest);

    let mut yaw: f32 = 0.0;
    let mut pitch: f32 = 0.0;

    let offset = vec2(30.0, 1.0);
    let width = atlas_texture.width();
    let height = atlas_texture.height();
    let block_size = 16.0;

    let mesh = Mesh {
        vertices: vec![
            Vertex::new(
                -0.5,
                0.5,
                0.5,
                offset.x * block_size / width,
                offset.y * block_size / height,
                WHITE,
            ),
            Vertex::new(
                0.5,
                0.5,
                0.5,
                (offset.x * block_size + block_size) / width,
                offset.y * block_size / height,
                WHITE,
            ),
            Vertex::new(
                0.5,
                -0.5,
                0.5,
                (offset.x * block_size + block_size) / width,
                (offset.y * block_size + block_size) / height,
                WHITE,
            ),
            Vertex::new(
                -0.5,
                -0.5,
                0.5,
                offset.x * block_size / width,
                (offset.y * block_size + block_size) / height,
                WHITE,
            ),
        ],
        indices: vec![0, 1, 3, 1, 2, 3],
        texture: Some(atlas_texture),
    };

    loop {
        set_camera(&camera);
        clear_background(BLACK);
        draw_mesh(&mesh);
        let delta = get_frame_time();
        let mouse_delta = mouse_delta_position();
        yaw += mouse_delta.x;
        pitch += mouse_delta.y;
        let limit = PI / 2.0 - 0.01;
        if pitch >= limit {
            pitch = limit
        }
        if pitch < -limit {
            pitch = -limit
        }
        let direction = Vec3 {
            x: yaw.sin() * pitch.cos(),
            y: pitch.sin(),
            z: yaw.cos() * pitch.cos(),
        };

        if is_key_down(KeyCode::W) {
            camera.position += direction * SPEED * delta;
        }

        if is_key_down(KeyCode::S) {
            camera.position -= direction * SPEED * delta;
        }

        let right = Vec3 {
            x: direction.z,
            y: 0.0,
            z: -direction.x,
        };

        if is_key_down(KeyCode::A) {
            camera.position += right * SPEED * delta;
        }
        if is_key_down(KeyCode::D) {
            camera.position -= right * SPEED * delta;
        }

        camera.target = camera.position + direction;
        set_default_camera();
        let fps_counter = format!("{} FPS", get_fps());
        draw_text(&fps_counter, 0.0, 25.0, 50.0, WHITE);
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        next_frame().await
    }
}
