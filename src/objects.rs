#[derive(Debug)]
pub enum Varient {
    Camera {
        lens_size: [f32; 2],
    },
    RigidBody {
        direction: [f32; 3], // The vector of the forces on the rigidbody
        size: [f32; 3],      // A rigidbody is always a cube
    },
    Mesh,
    Empty,
}

#[derive(Debug)]
pub struct Object {
    pub name: String,
    pub position: [f32; 3], // relative from parent
    pub rotation: [f32; 3], // relative from default (axis) in degrees
    pub varient: Varient,
    pub children: Vec<Object>,
}

impl Object {
    pub fn new(name: &str, varient: Varient) -> Object {
        Object {
            name: String::from(name),
            position: [0.0; 3],
            rotation: [0.0; 3],
            varient,
            children: Vec::new(),
        }
    }

    pub fn add_object(&mut self, mutable_object: Object) {
        let mut matching_names: Vec<usize> = Vec::new();
        let mut iter = 0;
        let mut matches = false;

        // Check whether there is no matching name under the same parent object
        while iter < self.children.len() {
            if self.children[iter].name[..mutable_object.name.len()] == mutable_object.name {
                matching_names.push(iter);
                matches = true;
            }
        }

        // WARNING: This code does not work properly, please do not use it
        if matches {
            let tries = 0;
            loop {
                for i in matching_names {
                    if self.children[i].name[mutable_object.name.len() - 1..]
                        == String::from(format!("{}", tries))
                    {
                        tries += 1;
                        continue;
                    }
                }

                mutable_object.name = mutable_object.name + format!("{}", tries);
                break;
            }
        }
        self.children.push(mutable_object);
    }

    pub fn print_all(&self) {
        //TODO: make this actually work
        println!("{:#?}", self);
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
