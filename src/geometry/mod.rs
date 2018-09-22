use super::*;

use self::node::*;

use self::types::*;
use gfx::traits::FactoryExt;
use glutin::dpi::*;
use gfx_window_glutin;
use self::gfx::Device;
use self::gfx::{Factory};
use self::glutin::{GlContext, GlRequest};
use self::glutin::Api::OpenGl;
use self::glutin::GlWindow;
use std::convert::AsMut;

gfx_defines!{

    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        color: [f32; 4] = "a_Color",
    }

    constant GeometryTransform {

        local: [[f32; 4]; 4] = "local_Transform",
        global: [[f32; 4]; 4] = "global_Transform",

    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        trans: gfx::ConstantBuffer<GeometryTransform> = "Transform",
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

pub struct GeometryRenderer {

    data: geometry::pipe::Data<ResourceType>,
    slice: gfx::Slice<ResourceType>,
    pipeline_state: gfx::PipelineState<ResourceType, geometry::pipe::Meta>

}

impl GeometryRenderer {

    pub fn new(data: geometry::pipe::Data<ResourceType>, slice: gfx::Slice<ResourceType>, pipeline_state: gfx::PipelineState<ResourceType, geometry::pipe::Meta>) -> GeometryRenderer {
        return GeometryRenderer { data, slice, pipeline_state };
    }

    pub fn from_vertices(vertices: &[Vertex], v_shader: &[u8], f_shader: &[u8], engine: &mut core::FlatEngine) -> GeometryRenderer {
        // Load shaders.
        let pipeline_state = engine.renderer.factory
            .create_pipeline_simple(
                v_shader,
                f_shader,
                pipe::new(),
            )
            .unwrap();

        let (vertex_buffer, slice) = engine.renderer.factory.create_vertex_buffer_with_slice(vertices, ());
        let trans_buffer = engine.renderer.factory.create_constant_buffer(1);
        let data = pipe::Data {
            vbuf: vertex_buffer,
            trans: trans_buffer,
            out: engine.renderer.render_view.clone(),
        };

        return GeometryRenderer::new(data, slice, pipeline_state);

    }

    // Automatically applies global transform to the render.
    pub fn render(&mut self, transform: Transform, engine: &mut core::FlatEngine) {
        engine.renderer.encoder.update_buffer(&self.data.trans, &[GeometryTransform { local: transform.data, global: engine.hints.global_trans.data }], 0); //update buffers
        engine.renderer.encoder.draw(&self.slice, &mut self.pipeline_state, &self.data); // draw commands with buffer data and attached pso
        engine.renderer.encoder.flush(engine.renderer.device.as_mut()); // execute draw commands
    }

}

pub struct Triangle {

    node: NodeObject,
    vertices: [Vertex; 3],
    color: Color,
    geometry_renderer: Option<GeometryRenderer>


}

impl Triangle {

    pub fn new(color: Color) -> Triangle {

        return Triangle {
            node: NodeObject::new(),
            vertices: [
                Vertex { pos: [ -0.5, -0.5], color: color.to_raw_color() },
                Vertex { pos: [  0.5, -0.5 ], color: color.to_raw_color() },
                Vertex { pos: [  0.0,  0.5], color: color.to_raw_color() },
            ],
            color: color,
            geometry_renderer: None

        };

    }

}

impl core::Drawable for Triangle {

    fn load(&mut self, engine: &mut core::FlatEngine) {
        self.geometry_renderer = Some(GeometryRenderer::from_vertices(&self.vertices, include_bytes!("../../shaders/triangle_v.glsl"), include_bytes!("../../shaders/triangle_f.glsl"), engine));
    }

    fn render(&mut self, engine: &mut core::FlatEngine) {

        // Check if all neccessary parts have been initialized.
        if self.geometry_renderer.is_some() {
            self.geometry_renderer.as_mut().unwrap().render(self.node.trans, engine);
        } else {
            panic!("The triangle object is being drawn before it has been initialized!");
        }

    }

    fn destroy(&mut self, engine: &mut core::FlatEngine) {

    }

}

impl Node for Triangle {

    fn set_pos(&mut self, pos: Vector2f) {
        self.node.set_pos(pos);
    }
    fn get_pos(&self) -> Vector2f {
        return self.node.get_pos();
    }

    fn set_trans(&mut self, trans: Transform) {
        self.node.set_trans(trans);
    }
    fn get_trans(&self) -> Transform {
        return self.node.get_trans();
    }

}
