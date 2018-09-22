use super::*;

use self::types::Transform;
use gfx::traits::FactoryExt;
use glutin::dpi::*;
use self::gfx::Device;
use self::gfx_device_gl::{Factory};
use gfx_window_glutin;
use self::glutin::{GlContext, GlRequest};
use self::glutin::Api::OpenGl;
use self::glutin::GlWindow;

use self::cgmath::*;

pub struct RenderHints {

    pub global_trans: Transform

}

impl RenderHints {

    pub fn new() -> RenderHints {

        return RenderHints { global_trans: Transform::identity() };

    }

}

pub struct Renderer {

    pub factory: Factory,
    pub encoder: gfx::Encoder<ResourceType, gfx_device_gl::CommandBuffer>,
    pub device: Box<gfx_device_gl::Device>,
    pub render_view: gfx::handle::RenderTargetView<ResourceType, (gfx::format::R8_G8_B8_A8, gfx::format::Unorm)>,
    pub depth_view: gfx::handle::DepthStencilView<ResourceType, (gfx::format::D24_S8, gfx::format::Unorm)>,

}

pub struct FlatEngine {

    pub renderer: Renderer,
    pub window: GlWindow,
    pub events_loop: glutin::EventsLoop,

    // Higher level data.
    pub hints: RenderHints

}

impl FlatEngine {

    /**
    Initialises the flat engine instance.
    */
    pub fn init(window_builder: glutin::WindowBuilder) -> FlatEngine {

        let mut events_loop = glutin::EventsLoop::new();

        let contextbuilder = glutin::ContextBuilder::new().with_gl(GlRequest::Specific(OpenGl,(3,2))).with_vsync(true);

        let (window, mut device, mut factory, color_view, mut depth_view) = gfx_window_glutin::init::<ColorFormat, DepthFormat>(window_builder, contextbuilder, &events_loop);

        let matrix: Matrix4<f32> = ortho(0.0, window.get_inner_size().unwrap().width as f32, 0.0, window.get_inner_size().unwrap().height as f32, 100.0, -100.0);

        let mut hints: RenderHints = RenderHints::new();

        hints.global_trans = Transform::from_matrix(matrix);

        // Put at the start of your file, outside of the loop
        let mut encoder: gfx::Encoder<ResourceType, gfx_device_gl::CommandBuffer> = factory.create_command_buffer().into();

        return FlatEngine {
            renderer: Renderer { factory: factory, encoder: encoder, device: Box::new(device), render_view: color_view, depth_view: depth_view },
            window: window,
            events_loop: events_loop,
            hints: hints
        };

    }

    pub fn clear(&mut self, color: Color) {

        self.renderer.encoder.clear(&self.renderer.render_view, color.to_raw_color()); //clear the framebuffer with a color(color needs to be an array of 4 f32s, RGBa)

    }

    pub fn swap_buffers(&mut self) {

        self.window.swap_buffers().unwrap();
        self.renderer.device.cleanup();

    }

    pub fn hande_event(&mut self, event: glutin::Event) {


        if let glutin::Event::WindowEvent { event, .. } = event {
            match event {
                glutin::WindowEvent::Resized(s) => {
                    gfx_window_glutin::update_views(&self.window, &mut self.renderer.render_view, &mut self.renderer.depth_view);
                }
                _ => (),
            }
        }
    }

    pub fn load(&mut self, drawable: &mut Drawable) {

        drawable.load(self)

    }

    pub fn render(&mut self, drawable: &mut Drawable) {

        drawable.render(self);

    }

    pub fn destroy(&mut self, drawable: &mut Drawable) {

        drawable.destroy(self);

    }

}

pub trait Drawable {

    fn load(&mut self, engine: &mut FlatEngine);

    fn render(&mut self, engine: &mut FlatEngine);

    fn destroy(&mut self, engine: &mut FlatEngine);

}