mod objects;

fn main() {
    let mut player = objects::RigidBody::RigidBody::new([0.0; 3], [0.0; 3], [0.0; 3], [0.0; 3]);
    player.children = vec![objects::objects::Mesh];
    println!("{:#?}", player);
}