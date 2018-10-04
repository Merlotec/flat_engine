// Note: This file should include all parts of the Flat Engine.
// When compiled for actual game use, this file will be renamed to lib.rs and a library will be compiled.
// The executable portion of this project is simply for testing.

#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
pub extern crate glutin;
pub extern crate image;
extern crate cgmath;
extern crate gfx_device_gl;
pub extern crate rusttype;
extern crate stopwatch;

pub mod core;
pub mod geometry;
pub mod types;
pub mod node;
pub mod render;
pub mod text;
pub mod spatial;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

pub type ResourceType = gfx_device_gl::Resources;
pub use self::types::*;
