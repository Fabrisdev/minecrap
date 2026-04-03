use macroquad::prelude::*;
use noise::NoiseFn;
use noise::Perlin;
use std::f32::consts::PI;

const SPEED: f32 = 15.0;
const CHUNK_SIZE: usize = 16;
const RENDER_DISTANCE: usize = 12;

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

    let up = vec3(0.0, 1.0, 0.0);

    let mut camera = Camera3D {
        up,
        fovy: 90.0,
        position: Vec3::ZERO,
        ..Default::default()
    };

    let atlas_texture = Texture2D::from_file_with_format(include_bytes!("assets/atlas.png"), None);
    atlas_texture.set_filter(FilterMode::Nearest);

    let mut yaw: f32 = 0.0;
    let mut pitch: f32 = 0.0;

    let mut meshes = vec![];
    for x in 0..RENDER_DISTANCE {
        for z in 0..RENDER_DISTANCE {
            let chunk = generate_chunk((x, z));
            let mesh = generate_chunk_mesh(&chunk, (x, z), &atlas_texture);
            meshes.extend(mesh)
        }
    }

    loop {
        set_camera(&camera);
        clear_background(BLACK);
        for mesh in &meshes {
            draw_mesh(&mesh);
        }

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

        let right = direction.cross(up).normalize();

        if is_key_down(KeyCode::A) {
            camera.position -= right * SPEED * delta;
        }
        if is_key_down(KeyCode::D) {
            camera.position += right * SPEED * delta;
        }

        camera.target = camera.position + direction;
        set_default_camera();
        let fps_counter = format!("{} FPS", get_fps());
        draw_text(&fps_counter, 0.0, 50.0, 100.0, WHITE);
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        next_frame().await
    }
}

fn generate_cube_mesh(position: Vec3, material_offset: Vec2, atlas_texture: &Texture2D) -> Mesh {
    let width = atlas_texture.width();
    let height = atlas_texture.height();
    let block_size = 16.0;

    let uvs = [
        (
            (material_offset.x * block_size + block_size) / width,
            material_offset.y * block_size / height,
        ),
        (
            material_offset.x * block_size / width,
            material_offset.y * block_size / height,
        ),
        (
            material_offset.x * block_size / width,
            (material_offset.y * block_size + block_size) / height,
        ),
        (
            (material_offset.x * block_size + block_size) / width,
            (material_offset.y * block_size + block_size) / height,
        ),
    ];

    let face_1 = [
        vec3(-0.5, 0.5, -0.5),
        vec3(0.5, 0.5, -0.5),
        vec3(0.5, -0.5, -0.5),
        vec3(-0.5, -0.5, -0.5),
    ];

    let face_2 = [
        vec3(0.5, 0.5, -0.5),
        vec3(0.5, 0.5, 0.5),
        vec3(0.5, -0.5, 0.5),
        vec3(0.5, -0.5, -0.5),
    ];

    let face_3 = [
        vec3(0.5, 0.5, 0.5),
        vec3(-0.5, 0.5, 0.5),
        vec3(-0.5, -0.5, 0.5),
        vec3(0.5, -0.5, 0.5),
    ];

    let face_4 = [
        vec3(-0.5, 0.5, 0.5),
        vec3(-0.5, 0.5, -0.5),
        vec3(-0.5, -0.5, -0.5),
        vec3(-0.5, -0.5, 0.5),
    ];

    let face_5 = [
        vec3(-0.5, 0.5, 0.5),
        vec3(0.5, 0.5, 0.5),
        vec3(0.5, 0.5, -0.5),
        vec3(-0.5, 0.5, -0.5),
    ];

    let face_6 = [
        vec3(-0.5, -0.5, -0.5),
        vec3(0.5, -0.5, -0.5),
        vec3(0.5, -0.5, 0.5),
        vec3(-0.5, -0.5, 0.5),
    ];

    let faces: Vec<[Vec3; 4]> = vec![face_1, face_2, face_3, face_4, face_5, face_6]
        .iter()
        .map(|face| face.map(|v| v + position))
        .collect();
    let amount_of_faces = faces.len();
    Mesh {
        vertices: faces_to_vertices(faces, uvs),
        indices: generate_mesh_indices(amount_of_faces),
        texture: Some(atlas_texture.clone()),
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

fn generate_chunk_mesh(
    chunk: &Chunk,
    offset: (usize, usize),
    atlas_texture: &Texture2D,
) -> Vec<Mesh> {
    let mut meshes = vec![];

    for (i, block) in chunk.blocks.iter().enumerate() {
        if let Some(material) = block.to_atlas_position() {
            let position = index_to_pos(i);
            let final_position = vec3(
                position.x + (CHUNK_SIZE * offset.0) as f32,
                position.y,
                position.z + (CHUNK_SIZE * offset.1) as f32,
            );
            let mesh = generate_cube_mesh(final_position, material, atlas_texture);
            meshes.push(mesh)
        }
    }

    meshes
}

#[derive(Clone, PartialEq, Debug)]
enum BlockType {
    AIR,
    GRASS,
    STONE,
}

impl BlockType {
    fn to_atlas_position(&self) -> Option<Vec2> {
        match &self {
            BlockType::AIR => None,
            BlockType::GRASS => Some(vec2(3.0, 0.0)),
            BlockType::STONE => Some(vec2(19.0, 0.0)),
        }
    }
}

#[derive(Debug)]
struct Chunk {
    blocks: Vec<BlockType>,
}

fn generate_chunk(offset: (usize, usize)) -> Chunk {
    let offset_x = (CHUNK_SIZE * offset.0) as f64;
    let offset_z = (CHUNK_SIZE * offset.1) as f64;
    let mut blocks = vec![BlockType::AIR; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE];
    let perlin = Perlin::new(1);
    let scale = 0.04;
    for x in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            let h = perlin.get([(x as f64 + offset_x) * scale, (z as f64 + offset_z) * scale]);
            let y = ((h + 1.0) / 2.0 * (CHUNK_SIZE - 1) as f64).floor();
            let index = pos_to_index(x, y as usize, z);
            blocks[index] = BlockType::GRASS;
        }
    }
    Chunk { blocks }
}

fn pos_to_index(x: usize, y: usize, z: usize) -> usize {
    x + y * CHUNK_SIZE + z * CHUNK_SIZE * CHUNK_SIZE
}

fn index_to_pos(i: usize) -> Vec3 {
    let x = i % CHUNK_SIZE;
    let y = (i / CHUNK_SIZE) % CHUNK_SIZE;
    let z = i / (CHUNK_SIZE * CHUNK_SIZE);
    vec3(x as f32, y as f32, z as f32)
}
