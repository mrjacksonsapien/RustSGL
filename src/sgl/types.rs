pub const RED: Color = Color { r: 255, g: 0, b: 0 };
pub const GREEN: Color = Color { r: 0, g: 255, b: 0 };
pub const BLUE: Color = Color { r: 0, g: 0, b: 255 };

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct Transform {
    pub position: Vector3,
    pub rotation: Vector3,
    pub scale: Vector3,
}

pub struct Vertex {
    pub position: Vector3,
    pub color: Color,
}

pub struct Triangle {
    pub indices: [usize; 3],
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub triangles: Vec<Triangle>,
    pub transform: Transform,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, triangles: Vec<Triangle>) -> Self {
        Self {
            vertices,
            triangles,
            transform: Transform {
                position: Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                rotation: Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                scale: Vector3 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                },
            },
        }
    }
}

struct Cube;

impl Cube {
    pub fn new() -> Mesh {
        // TODO: Add cube data
        let vertices = vec![];

        let triangles = vec![];

        Mesh::new(vertices, triangles)
    }
}

pub struct Camera {
    pub near: f32,
    pub far: f32,
    pub fov: f32,
    pub transform: Transform,
}

pub struct Scene {
    meshes: Vec<Mesh>,
    current_camera: Camera,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            meshes: Vec::new(),
            current_camera: Camera {
                near: 0.1,
                far: 100.0,
                fov: 90.0,
                transform: Transform {
                    position: Vector3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    rotation: Vector3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    scale: Vector3 {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    },
                },
            },
        }
    }
}
