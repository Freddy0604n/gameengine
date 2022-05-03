#[macro_use]
extern crate glium;

use objects::ObjectVarient;

pub mod objects;
mod stage;

fn main() {
    let mut world = objects::World::new();
    world.add_object("Player", 0, ObjectVarient::Empty);
    println!("{:?}", world);
    world.print_all();
}
