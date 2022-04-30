pub mod rigid_body {
    #[derive(Debug)]
    pub struct RigidBody {
        position: [f64; 3],
        direction: [f64; 3],
        shape: [f64; 3]     // a rigidbody is alway a cube
    }

    impl RigidBody {
        pub fn new(position: [f64; 3], direction: [f64; 3], shape: [f64; 3]) -> RigidBody {
            RigidBody {
                position,
                direction,
                shape
            }
        }
    }
}