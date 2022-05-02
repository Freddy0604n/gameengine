pub struct Camera {
    pub position: [f64; 3], // relative from parent
    pub rotation: [f64; 3], // rotation is in degrees from the axis
    pub lens_size: [f64; 2],
    pub lens_distance: f64,
    pub id: u32
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