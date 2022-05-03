#[derive(Debug)]
pub struct World {
    // the parent of all objects
    pub id: u32,
    pub children: Vec<u32>,
    pub id_tracker: u32,
    pub name: String,
    pub content: Vec<Object>,
}

impl World {
    pub fn new() -> World {
        World {
            id: 0,
            children: Vec::new(),
            id_tracker: 1,
            name: String::from("main"),
            content: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.id_tracker = 0;
        self.children = Vec::new();
    }

    pub fn add_object(&mut self, name: &str, parent_id: u32, varient: ObjectVarient) {
        let new = Object::new(name, parent_id, varient, self.id_tracker);
        let mut iter = 0;
        if parent_id != 0 {
            loop {
                // change the children_ids vector in the parent to include the new object
                if self.content[iter].id == parent_id {
                    self.content[iter].children_ids.push(self.id_tracker);
                    break;
                }
                iter += 1;
            }
        } else {
            self.children.push(self.id_tracker);
        }
        self.id_tracker += 1;
        self.content.push(new);
    }

    pub fn print_all(&self) {
        // This is a function that does not function 
       for i in self.content.iter() {
            if i.id == 0 {
                println!("{}", i.name);
                let under = self.get_children(i);
                for j in under {
                    if j.children_ids == Vec::new() {
                        println!("|__{}", j.name);
                    }
                }
            }
       }
    }

    fn get_children(&self, object: &Object) -> Vec<&Object> {
        // This function does not yet function the way I want it to
        let mut returnval = Vec::new();
        for ids in object.children_ids.iter() {
            for option in self.content.iter() {
                if option.id == *ids {
                    returnval.push(option);
                    break;
                }
            }
        }
        returnval
    }
}
#[derive(Debug)]
pub enum ObjectVarient {
    Camera,
    RigidBody,
    Mesh,
    Empty,
}
#[derive(Debug)]
pub struct Object {
    pub name: String,
    pub position: [f32; 3], // relative from parent
    pub rotation: [f32; 3], // relative from default (axis) in degrees
    pub id: u32,            // for keeping track of the child-parent structure
    pub varient: ObjectVarient,
    pub parent_id: u32,
    pub children_ids: Vec<u32>,
}

impl Object {
    fn new(name: &str, parent_id: u32, varient: ObjectVarient, id: u32) -> Object {
        Object {
            name: String::from(name),
            position: [0.0; 3],
            rotation: [0.0; 3],
            id,
            varient,
            parent_id,
            children_ids: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: &Object) {
        // adds the id of the given child to the parent vector after checking parent id
        if child.parent_id != self.id {
            panic!("The child parent id does not match the id of the parent you want to assign to");
        }
        self.children_ids.push(child.id);
    }

    pub fn move_with(&mut self, change: [f32; 3]) {
        // move the object to new place relative of current position
        let mut iter = 0;
        while iter < 3 {
            self.position[iter] += change[iter];
            iter += 1;
        }
    }
}
