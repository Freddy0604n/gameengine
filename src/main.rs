use objects::ObjectVarient;

pub mod objects;
mod world;

fn main() {
    let mut world = objects::World::new();
    world.add_object("Player", 0, ObjectVarient::Empty);
    println!("{:?}", world);
}
