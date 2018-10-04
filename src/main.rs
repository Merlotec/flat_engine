// Note: This file should include all parts of the Flat Engine.
// When compiled for actual game use, this file will be renamed to lib.rs and a library will be compiled.
// The executable portion of this project is simply for testing.

#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate image;
extern crate cgmath;
extern crate gfx_device_gl;
extern crate rusttype;
extern crate stopwatch;

mod core;
mod geometry;
mod types;
mod node;
mod render;
mod text;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

pub type ResourceType = gfx_device_gl::Resources;

// THE FOLLOWING CODE IS TESTING CODE -- NOT PART OF LIBRARY DISTRIBUTION

pub use self::types::*;
use self::geometry::Triangle;
use self::render::*;
use self::node::*;
use self::core::Drawable;
use self::text::*;
use self::std::time::Instant;
use self::std::str::*;

const CLEAR_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];



fn main() {

    let window_builder = glutin::WindowBuilder::new().with_title("Flat Engine v 0.1".to_string()).with_fullscreen(None).with_resizable(false);

    let mut engine: core::FlatEngine = core::FlatEngine::init(window_builder);

    let mut texture: Texture = Texture::load_from_path("resources/logo.png");
    let mut logo: Sprite = Sprite::new(Some(texture));

    let mut text: Text = Text::new("Flat Engine v 0.1", Font::from_bytes(include_bytes!("../resources/trebuc.ttf") as &[u8]).unwrap(), 40.0, Color::white());

    text.set_pos(Vector2f::new(0.0, 0.0));

    logo.set_pos(Vector2f::new(0.0, 0.0));
    logo.set_size(Vector2f::new(engine.window.get_inner_size().unwrap().width, engine.window.get_inner_size().unwrap().height));

    engine.load(&mut text);
    engine.load(&mut logo);

    let mut stopwatch: stopwatch::Stopwatch = stopwatch::Stopwatch::start_new();

    let mut running = true;
    while running {

        let p = logo.get_pos();

        let delta: f64 = stopwatch.elapsed_ms() as f64 / 1000.0;
        stopwatch.reset();
        stopwatch.start();

        let mut fps_text = ((1.0 / delta) as i64).to_string().clone();

        if (fps_text != text.text) {
            text.set_text(fps_text);
        }

        let mut offset: f64 = 0.0;

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
                    glutin::WindowEvent::KeyboardInput {
                        input:
                        glutin::KeyboardInput {
                            virtual_keycode: Some(glutin::VirtualKeyCode::Right),
                            ..
                        },
                        ..
                    } => offset = delta * 100.0,
                    glutin::WindowEvent::KeyboardInput {
                        input:
                        glutin::KeyboardInput {
                            virtual_keycode: Some(glutin::VirtualKeyCode::Left),
                            ..
                        },
                        ..
                    } => offset = delta * -100.0,
                    _ => (),
                }
            }
        });

        logo.set_pos(Vector2::new(p.x + offset, p.y));

        engine.clear(Color::black());
        // Render code goes here.
        engine.render(&mut logo);
        engine.render(&mut text);
        engine.swap_buffers();

    }
}