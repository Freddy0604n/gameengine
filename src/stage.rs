/*
Varient representation:
    10 -> Camera,
    20 -> RigidBody,
    30 -> Mesh,
    40 -> Empty,
*/

use crate::objects::*;
use std::fs;

pub fn load(path: &str) -> Object {
    let array = fs::read(path);
    Object::new("Placeholder", Varient::Empty)
}

pub fn convert(object: Object) -> Vec<u8> {
    // converts an object including its children into the .stg file format, in a vector of bytes
    let mut returnval: Vec<u8> = Vec::new();

    returnval
}

fn convert_single(object: Object) -> Vec<u8> {
    // converts a single object excluding its children
    let mut returnval: Vec<u8> = Vec::new();
    returnval.push(0); // start of object
    for i in object.name.as_bytes() {
        returnval.push(*i);
    }
    returnval.push(b';'); // end of name value
    for i in object.position {
        for j in i.to_be_bytes() {
            returnval.push(j);
        }
    }
    returnval.push(b';'); // end of position value
    for i in object.rotation {
        for j in i.to_be_bytes() {
            returnval.push(j);
        }
    }
    returnval.push(b';');
    let typevar: u8 = 0;
    match object.varient {
        Camera => typevar = 10,
        RigidBody => typevar = 20,
        Mesh => typevar = 30,
        Empty => typevar = 40,
    }
    returnval.push(typevar);
    returnval.push(b';'); // end of object type value
    if typevar == 10 || typevar == 20 {
        // check whether additional data should be added
        let add_data: Vec<u8> = Vec::new();
        match typevar {
            10 => {
                for i in object.varient {
                    for j in i.to_be_bytes() {
                        returnval.push(j);
                    }
                }
            }
        }
    }
    returnval.push(0); // end of object
    returnval
}
