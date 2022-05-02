pub enum ObjectVarient {
    Camera,
    RigidBody,
    Mesh
}
pub struct Object {
    pub position: [f32; 3], // relative from parent
    pub rotation: [f32; 3], // relative from default (axis) in degrees
    pub id: u32, // fro keeping track of the child-parent structure
    pub varient: ObjectVarient
}
