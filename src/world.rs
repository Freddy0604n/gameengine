use crate::objects::*;
use std::fs;

fn load(path: &str) -> Vec<Object> {
    let file = fs::read(path);
    vec![Object {
        name: String::from("Test"),
        position: [0.0; 3],
        rotation: [0.0; 3],
        id: 0,
        varient: ObjectVarient::Empty,
        parent_id: 0,
        children_ids: Vec::new(),
    }]
}
/*  The .wrld file:
List of all objects in a world excluding the world object itself.
Different objects are seperated by a \n, different pieces of data are seperated by NUL.
*/
fn convert(world: Vec<Object>) -> Vec<u8> {
    let mut writedata: Vec<u8> = Vec::new();
    for i in world {
        for chars in i.name.as_bytes() {
            writedata.push(*chars);
        }
        writedata.push(0); // seperator
    }
    writedata
}
