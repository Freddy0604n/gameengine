pub struct World {
    pub id: u32,
    pub children: Vec<u32>,
    pub id_tracker: u32
}

impl World {
    pub fn new() -> World {
        World {
            id: 0,
            children: Vec::new(),
            id_tracker: 0
        }
    }
}
#[derive(Debug)]
pub enum ObjectVarient {
    Camera,
    RigidBody,
    Mesh,
    Empty
}
#[derive(Debug)]
pub struct Object {
    pub position: [f32; 3], // relative from parent
    pub rotation: [f32; 3], // relative from default (axis) in degrees
    pub id: u32, // for keeping track of the child-parent structure
    pub varient: ObjectVarient,
    pub parent_id: u32,
    pub children_ids: Vec<u32>
}

impl Object {
    pub fn new(parent_id: u32, varient: ObjectVarient, world: &mut World) -> Object {
        world.id_tracker += 1;
        Object {
            position: [0.0; 3],
            rotation: [0.0; 3],
            id: world.id_tracker,
            varient,
            parent_id,
            children_ids: Vec::new()
        }
    }

    pub fn add_child(&mut self, child: &Object) {   // adds the id of the given child to the parent vector after checking parent id
        if child.parent_id != self.id {
            panic!("The child parent id does not match the id of the parent you want to assign to");
        }
        self.children_ids.push(child.id);
    }
}