// Note: This file should include all parts of the Flat Engine.
// When compiled for actual game use, this file will be renamed to lib.rs and a library will be compiled.
// The executable portion of this project is simply for testing.

#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;

extern crate gfx_device_gl;

mod core;
mod geometry;
mod types;
mod node;
mod sprite;

// THE FOLLOWING CODE IS TESTING CODE -- NOT PART OF LIBRARY DISTRIBUTION

pub use self::types::*;
use self::geometry::Triangle;

const CLEAR_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];



fn main() {

    let window_builder = glutin::WindowBuilder::new().with_title("Flat Engine v 0.0.1".to_string()).with_dimensions(glutin::dpi::LogicalSize { width: 512.0, height: 512.0 } );

    let mut engine: core::FlatEngine = core::FlatEngine::init(window_builder);

    let mut triangle: Triangle = Triangle::new(Color::red());

    engine.load(&mut triangle);

    let mut running = true;
    while running {
        // fetch events

        engine.events_loop.poll_events(|event| {
            if let glutin::Event::WindowEvent { event, .. } = event {
                match event {
                    glutin::WindowEvent::CloseRequested => running = false,
                    glutin::WindowEvent::KeyboardInput {
                        input:
                        glutin::KeyboardInput {
                            virtual_keycode: Some(glutin::VirtualKeyCode::Escape),
                            ..
                        },
                        ..
                    } => return,
                    _ => (),
                }
            }
        });
        engine.clear(Color::black());
        engine.render(&mut triangle);
        engine.swap_buffers();

    }
}