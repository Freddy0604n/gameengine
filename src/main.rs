#[macro_use]
extern crate glium;
use glium::glutin;
use glium::Surface;

use objects::ObjectVarient;

pub mod objects;
mod stage;

fn main() {
    let mut world = objects::World::new();
    world.add_object("Player", 0, ObjectVarient::Empty);
    println!("{:?}", world);
    world.print_all();

    let mut event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    event_loop.run(move |ev, _, control_flow| {

        let mut target = display.draw();
        target.clear_color(1.0, 1.0, 0.0, 0.0);
        target.finish().unwrap();

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            _ => (),
        }
    });
}
