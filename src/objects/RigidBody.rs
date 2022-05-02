#[derive(Debug)]
pub struct RigidBody {
    pub position: [f64; 3], // relative from parent
    pub rotation: [f64; 3], // rotation is in degrees from the axis
    pub direction: [f64; 3],
    pub shape: [f64; 3],     // a rigidbody is alway a cube
    pub id: u32
}

impl RigidBody {
    pub fn new(position: [f64; 3], direction: [f64; 3], rotation: [f64; 3], shape: [f64; 3], id: u32) -> RigidBody {
        RigidBody {
            position,
            direction,
            rotation,
            shape,
            id
        }
    }
}