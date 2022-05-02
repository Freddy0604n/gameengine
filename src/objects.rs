pub mod RigidBody;

pub enum Objects {
    Camera,
    Mesh,
    Rigidbody
}

pub struct Camera {
    position: [f64; 3],
    rotation: [f64; 3], // rotation is in degrees from the axis
    lens_size: [f64; 2],
    lens_distance: f64
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            position: [0.0; 3],
            rotation: [0.0; 3],
            lens_size: [2.5, 1.5],
            lens_distance: 1.0
        }
    }
}