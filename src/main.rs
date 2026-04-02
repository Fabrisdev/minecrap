use std::f32::consts::PI;

use macroquad::prelude::*;

const SPEED: f32 = 4.0;

fn window_config() -> Conf {
    Conf {
        window_title: "Minecrap".to_string(),
        platform: {
            miniquad::conf::Platform {
                swap_interval: Some(0),
                ..Default::default()
            }
        },
        ..Default::default()
    }
}
#[macroquad::main(window_config())]
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

    let mesh = generate_cube_mesh(vec3(0.0, 0.0, 3.0), vec2(30.0, 1.0), atlas_texture);

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
        }; /* 
        let direction = Vec3 {
        x: yaw.cos(),
        y: pitch.sin(),
        z: -yaw.sin(),
        };*/

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

fn generate_cube_mesh(position: Vec3, material_offset: Vec2, atlas_texture: Texture2D) -> Mesh {
    let width = atlas_texture.width();
    let height = atlas_texture.height();
    let block_size = 16.0;

    let uvs = [
        (
            material_offset.x * block_size / width,
            material_offset.y * block_size / height,
        ),
        (
            (material_offset.x * block_size + block_size) / width,
            material_offset.y * block_size / height,
        ),
        (
            (material_offset.x * block_size + block_size) / width,
            (material_offset.y * block_size + block_size) / height,
        ),
        (
            material_offset.x * block_size / width,
            (material_offset.y * block_size + block_size) / height,
        ),
    ];

    let face_1 = [
        vec3(-0.5, 0.5, 0.5) + position,
        vec3(0.5, 0.5, 0.5) + position,
        vec3(0.5, -0.5, 0.5) + position,
        vec3(-0.5, -0.5, 0.5) + position,
    ];

    let face_2 = [
        vec3(0.5, 0.5, -0.5) + position,
        vec3(0.5, 0.5, 0.5) + position,
        vec3(0.5, -0.5, 0.5) + position,
        vec3(0.5, -0.5, -0.5) + position,
    ];

    let faces = vec![face_1, face_2];
    let amount_of_faces = faces.len();
    Mesh {
        vertices: faces_to_vertices(faces, uvs),
        indices: generate_mesh_indices(amount_of_faces),
        texture: Some(atlas_texture),
    }
}

fn face_to_vertices(vertices: [Vec3; 4], uvs: [(f32, f32); 4]) -> Vec<Vertex> {
    vertices
        .iter()
        .zip(uvs)
        .map(|(v, uv)| Vertex::new(v.x, v.y, v.z, uv.0, uv.1, WHITE))
        .collect()
}

fn faces_to_vertices(faces: Vec<[Vec3; 4]>, uvs: [(f32, f32); 4]) -> Vec<Vertex> {
    faces
        .iter()
        .flat_map(|face| face_to_vertices(*face, uvs))
        .collect()
}

fn generate_mesh_indices(amount_of_faces: usize) -> Vec<u16> {
    let mut indices: Vec<u16> = Vec::with_capacity(amount_of_faces * 6);
    for i in 0..amount_of_faces {
        let offset = i as u16 * 4;
        indices.extend_from_slice(&[
            offset,
            offset + 1,
            offset + 2,
            offset,
            offset + 2,
            offset + 3,
        ]);
    }
    indices
}
