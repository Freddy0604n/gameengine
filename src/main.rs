mod objects;

fn main() {
    let mut world = objects::World::new();
    let mut player = objects::Object::new(0, objects::ObjectVarient::Empty, &mut world);
    let camera = objects::Object::new(player.id, objects::ObjectVarient::Camera, &mut world);
    player.add_child(&camera);

    println!("{:?}\n{:?}", player, camera);
}