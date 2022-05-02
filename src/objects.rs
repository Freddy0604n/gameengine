pub struct IdLedger {
    ledger: Vec<>
}

impl IdLedger {
    pub fn get_id(&self, parent: u32) {
        
    }
}

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

impl Object {
    fn new(parent_id: u32, varient: ObjectVarient, ledger: Vec<u32>) -> Object {
        Object {
            position: [0.0; 3],
            rotation: [0.0; 3],
            id: 1,
            varient
        }
    }
}