use super::*;

use self::types::*;
use gfx::traits::FactoryExt;
use glutin::dpi::*;
use self::gfx::Device;
use self::gfx_device_gl::{Factory};
use gfx_window_glutin;
use self::glutin::{GlContext, GlRequest};
use self::glutin::Api::OpenGl;
use self::glutin::GlWindow;

use self::cgmath::Matrix4;

pub struct Renderer {

    pub factory: Factory,
    pub encoder: gfx::Encoder<ResourceType, gfx_device_gl::CommandBuffer>,
    pub device: Box<gfx_device_gl::Device>,
    pub render_view: gfx::handle::RenderTargetView<ResourceType, (gfx::format::R8_G8_B8_A8, gfx::format::Unorm)>,
    pub depth_view: gfx::handle::DepthStencilView<ResourceType, (gfx::format::D24_S8, gfx::format::Unorm)>,

    pub camera: Camera,

}

pub struct Camera {

    pub projection: Matrix4f,
    pub view: Matrix4f,

}

impl Camera {

    pub fn ortho(size: Vector2f) -> Camera {

        return Camera {
            projection: cgmath::ortho(0.0, size.x, 0.0, size.y, 100.0, -100.0),
            view: Matrix4f::identity()
        };

    }

    pub fn set_pos(&mut self, pos: Vector3f) {

        self.view.w.x = -pos.x;
        self.view.w.y = -pos.y;
        self.view.w.z = -pos.z;

    }

    pub fn get_pos(&self) -> Vector3f {

        return Vector3f { x: -self.view.w.x, y: -self.view.w.y, z: -self.view.w.z };

    }

}

pub struct FlatEngine {

    pub renderer: Renderer,
    pub window: GlWindow,
    pub events_loop: glutin::EventsLoop,

}

impl FlatEngine {

    /**
    Initialises the flat engine instance.
    */
    pub fn init(window_builder: glutin::WindowBuilder) -> FlatEngine {

        let mut events_loop = glutin::EventsLoop::new();

        let contextbuilder = glutin::ContextBuilder::new().with_gl(GlRequest::Specific(OpenGl,(3,2))).with_vsync(true);

        let (window, mut device, mut factory, color_view, mut depth_view) = gfx_window_glutin::init::<ColorFormat, DepthFormat>(window_builder, contextbuilder, &events_loop);

        let window_size: Vector2f = Vector2f { x: window.get_inner_size().unwrap().width as f32, y: window.get_inner_size().unwrap().height as f32 };

        // Put at the start of your file, outside of the loop
        let mut encoder: gfx::Encoder<ResourceType, gfx_device_gl::CommandBuffer> = factory.create_command_buffer().into();

        return FlatEngine {
            renderer: Renderer { factory: factory, encoder: encoder, device: Box::new(device), render_view: color_view, depth_view: depth_view, camera: Camera::ortho(window_size) },
            window: window,
            events_loop: events_loop
        };

    }

    pub fn clear(&mut self, color: Color) {

        self.renderer.encoder.clear(&self.renderer.render_view, color.to_raw_color()); //clear the framebuffer with a color(color needs to be an array of 4 f32s, RGBa)

    }

    pub fn swap_buffers(&mut self) {
        self.window.swap_buffers().unwrap();
        self.renderer.device.cleanup();

    }

    pub fn flush(&mut self) {
        self.renderer.encoder.flush(self.renderer.device.as_mut());
    }

    pub fn update_size(&mut self) {

        gfx_window_glutin::update_views(&self.window, &mut self.renderer.render_view, &mut self.renderer.depth_view);

    }

    pub fn get_dimensions(&self) -> Vector2f {

        let size = self.window.get_inner_size().unwrap();
        return Vector2f { x: size.width as f32, y: size.height as f32 };

    }

    pub fn scale(&self, scale: Vector2f) -> Vector2f {

        let d: Vector2f = self.get_dimensions();

        return Vector2f::new(d.x * scale.x, d.y * scale.y);

    }

    pub fn layout_sized_from_center(&self, node: &mut node::SizedNode2D, normalized_pos: Vector2f) {

        let x: f32 = (self.get_dimensions().x * normalized_pos.x) - (node.get_size().x / 2.0);

        let y: f32 = (self.get_dimensions().y * normalized_pos.y) - (node.get_size().y / 2.0);

        node.set_pos(Vector2f { x, y });

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