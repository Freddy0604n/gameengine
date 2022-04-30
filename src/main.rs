mod physics;

fn main() {
    let player = physics::rigid_body::RigidBody::new([0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]);
    println!("{:#?}", player);
}