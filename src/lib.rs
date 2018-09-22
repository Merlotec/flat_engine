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
