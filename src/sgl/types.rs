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
}

pub struct Triangle {
    pub indices: [usize; 3],
    pub color: Color,
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub triangles: Vec<Triangle>,
    pub transform: Transform,
}

impl Mesh {
    fn new(vertices: Vec<Vertex>, triangles: Vec<Triangle>) -> Self {
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
    fn new() -> Mesh {
        // 1. Define the 8 unique corners of the cube
        let vertices = vec![
            Vertex {
                position: Vector3 {
                    x: -0.5,
                    y: -0.5,
                    z: 0.5,
                },
            }, // 0: Front-Bottom-Left
            Vertex {
                position: Vector3 {
                    x: 0.5,
                    y: -0.5,
                    z: 0.5,
                },
            }, // 1: Front-Bottom-Right
            Vertex {
                position: Vector3 {
                    x: 0.5,
                    y: 0.5,
                    z: 0.5,
                },
            }, // 2: Front-Top-Right
            Vertex {
                position: Vector3 {
                    x: -0.5,
                    y: 0.5,
                    z: 0.5,
                },
            }, // 3: Front-Top-Left
            Vertex {
                position: Vector3 {
                    x: -0.5,
                    y: -0.5,
                    z: -0.5,
                },
            }, // 4: Back-Bottom-Left
            Vertex {
                position: Vector3 {
                    x: 0.5,
                    y: -0.5,
                    z: -0.5,
                },
            }, // 5: Back-Bottom-Right
            Vertex {
                position: Vector3 {
                    x: 0.5,
                    y: 0.5,
                    z: -0.5,
                },
            }, // 6: Back-Top-Right
            Vertex {
                position: Vector3 {
                    x: -0.5,
                    y: 0.5,
                    z: -0.5,
                },
            }, // 7: Back-Top-Left
        ];

        // 2. Define the triangles using the indices of the vertices above
        // Each face has 2 triangles.
        let triangles = vec![
            // Front Face (using RED)
            Triangle {
                indices: [0, 1, 2],
                color: RED,
            },
            Triangle {
                indices: [0, 2, 3],
                color: RED,
            },
            // Right Face (using GREEN)
            Triangle {
                indices: [1, 5, 6],
                color: GREEN,
            },
            Triangle {
                indices: [1, 6, 2],
                color: GREEN,
            },
            // Back Face
            Triangle {
                indices: [5, 4, 7],
                color: BLUE,
            },
            Triangle {
                indices: [5, 7, 6],
                color: BLUE,
            },
            // Left Face
            Triangle {
                indices: [4, 0, 3],
                color: GREEN,
            },
            Triangle {
                indices: [4, 3, 7],
                color: GREEN,
            },
            // Top Face
            Triangle {
                indices: [3, 2, 6],
                color: RED,
            },
            Triangle {
                indices: [3, 6, 7],
                color: RED,
            },
            // Bottom Face
            Triangle {
                indices: [4, 5, 1],
                color: BLUE,
            },
            Triangle {
                indices: [4, 1, 0],
                color: BLUE,
            },
        ];

        Mesh::new(vertices, triangles)
    }
}

pub struct Camera {
    pub near: f32,
    pub far: f32,
    pub fov: f32,
    pub position: Vector3,
    pub orientation: Vector3,
}

pub struct Scene {
    meshes: Vec<Mesh>,
    current_camera: Camera,
}

pub struct Renderer {
    scene: Scene,
}
