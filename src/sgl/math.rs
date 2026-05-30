use crate::sgl::types::{Camera, Vector3};
use wide::*;

const SIZEOF_TRIANGLE_DATA: usize = 3;
const SIZEOF_VERTEX_DATA: usize = 7;

pub fn cot(degrees: f32) -> f32 {
    return 1.0 / degrees.to_radians().tan();
}

fn multiply_4x4_matrices(a: [f32; 16], b: [f32; 16]) -> [f32; 16] {
    let mut out = [0.0; 16];

    let b0 = f32x4::from(&b[0..4]);
    let b1 = f32x4::from(&b[4..8]);
    let b2 = f32x4::from(&b[8..12]);
    let b3 = f32x4::from(&b[12..16]);

    for i in 0..4 {
        let row_offset = i * 4;

        let ax = f32x4::splat(a[row_offset + 0]);
        let ay = f32x4::splat(a[row_offset + 1]);
        let az = f32x4::splat(a[row_offset + 2]);
        let aw = f32x4::splat(a[row_offset + 3]);

        let res = (ax * b0) + (ay * b1) + (az * b2) + (aw * b3); // Runs like SIMD

        out[row_offset..row_offset + 4].copy_from_slice(&res.to_array());
    }
    out
}

fn create_translation_matrix(x: f32, y: f32, z: f32) -> [f32; 16] {
    [
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, x, y, z, 1.0,
    ]
}

fn create_scale_matrix(x: f32, y: f32, z: f32) -> [f32; 16] {
    [
        x, 0.0, 0.0, 0.0, 0.0, y, 0.0, 0.0, 0.0, 0.0, z, 0.0, 0.0, 0.0, 0.0, 1.0,
    ]
}

fn create_pitch_matrix(pitch_degrees: f32) -> [f32; 16] {
    let pitch = pitch_degrees.to_radians();
    let cos = pitch.cos();
    let sin = pitch.sin();

    [
        1.0, 0.0, 0.0, 0.0, 0.0, cos, -sin, 0.0, 0.0, sin, cos, 0.0, 0.0, 0.0, 0.0, 1.0,
    ]
}

fn create_yaw_matrix(yaw_degrees: f32) -> [f32; 16] {
    let yaw = yaw_degrees.to_radians();
    let cos = yaw.cos();
    let sin = yaw.sin();

    [
        cos, 0.0, sin, 0.0, 0.0, 1.0, 0.0, 0.0, -sin, 0.0, cos, 0.0, 0.0, 0.0, 0.0, 1.0,
    ]
}

fn create_roll_matrix(roll_degrees: f32) -> [f32; 16] {
    let roll = roll_degrees.to_radians();
    let cos = roll.cos();
    let sin = roll.sin();

    [
        cos, -sin, 0.0, 0.0, sin, cos, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    ]
}

fn create_euler_matrix(pitch_degrees: f32, yaw_degrees: f32, roll_degrees: f32) -> [f32; 16] {
    multiply_4x4_matrices(
        multiply_4x4_matrices(
            create_yaw_matrix(yaw_degrees),
            create_pitch_matrix(pitch_degrees),
        ),
        create_roll_matrix(roll_degrees),
    )
}

fn create_transformation_matrix(
    position: &Vector3,
    rotation: &Vector3,
    scale: &Vector3,
) -> [f32; 16] {
    let scale_matrix = create_scale_matrix(scale.x, scale.y, scale.z);
    let rotation_matrix = create_euler_matrix(rotation.x, rotation.y, rotation.z);
    let translation_matrix = create_translation_matrix(position.x, position.y, position.z);

    multiply_4x4_matrices(
        multiply_4x4_matrices(scale_matrix, rotation_matrix),
        translation_matrix,
    )
}

fn create_view_matrix(camera: &Camera) -> [f32; 16] {
    multiply_4x4_matrices(
        create_translation_matrix(
            camera.transform.position.x,
            camera.transform.position.y,
            camera.transform.position.z,
        ),
        create_euler_matrix(
            camera.transform.rotation.x,
            camera.transform.rotation.y,
            camera.transform.rotation.z,
        ),
    )
}

fn create_projection_matrix(camera: &Camera) -> [f32; 16] {
    let aspect_ratio: f32 = 1.0; // TODO: Implement actual aspect ratio
    let cot_div_aspect = cot(camera.fov / 2.0) / aspect_ratio;

    [
        cot_div_aspect,
        0.0,
        0.0,
        0.0,
        0.0,
        cot_div_aspect,
        0.0,
        0.0,
        0.0,
        0.0,
        -(camera.far / (camera.far - camera.near)),
        -1.0,
        0.0,
        0.0,
        (camera.far * camera.near) / (camera.far - camera.near),
        0.0,
    ]
}

fn multiply_matrix_with_vertices(m: &[f32; 16], vertices_data: &mut [f32]) {
    let col0 = f32x4::from([m[0], m[4], m[8], m[12]]);
    let col1 = f32x4::from([m[1], m[5], m[9], m[13]]);
    let col2 = f32x4::from([m[2], m[6], m[10], m[14]]);
    let col3 = f32x4::from([m[3], m[7], m[11], m[15]]);

    for vertex in vertices_data.chunks_exact_mut(SIZEOF_VERTEX_DATA) {
        let x = f32x4::splat(vertex[0]);
        let y = f32x4::splat(vertex[1]);
        let z = f32x4::splat(vertex[2]);
        let w = f32x4::splat(vertex[3]);

        let res = (x * col0) + (y * col1) + (z * col2) + (w * col3);

        vertex.copy_from_slice(&res.to_array());
    }
}
