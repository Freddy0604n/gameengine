#[macro_use]
extern crate glium;
use glium::glutin;
use glium::Surface;

use objects::*;

pub mod objects;
mod stage;
mod network;

fn main() {
    let mut world = Object::new("World", Varient::Empty);
    world.add_object(Object::new("Player", Varient::Empty));
    world.children[0].add_object(Object::new(
        "rigidbody",
        Varient::RigidBody {
            direction: [0.0; 3],
            size: [1.0; 3],
        },
    ));

    let mut event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    event_loop.run(move |ev, _, control_flow| {
        let mut target = display.draw();
        target.clear_color(1.0, 1.0, 0.0, 0.0);
        target.finish().unwrap();
        println!("cool");

        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            _ => (),
        }
    });
}
