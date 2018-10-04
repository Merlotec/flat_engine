use super::*;

use node::*;
use gfx::traits::FactoryExt;

gfx_defines!{

    vertex UvVertex3f {
        pos: [f32; 3] = "a_Pos",
        uv: [f32; 2] = "a_Uv",
    }

    constant MeshTransform {

        model: [[f32; 4]; 4] = "model_Transform",
        view: [[f32; 4]; 4] = "view_Transform",
        projection: [[f32; 4]; 4] = "projection_Transform",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<UvVertex3f> = (),
        tex: gfx::TextureSampler<[f32; 4]> = "t_Texture",
        trans: gfx::ConstantBuffer<MeshTransform> = "Transform",
        out: gfx::BlendTarget<ColorFormat> = ("Target0", gfx::state::ColorMask::all(), gfx::preset::blend::ALPHA),
    }
}

pub struct MeshRenderer {

    data: spatial::pipe::Data<ResourceType>,
    slice: gfx::Slice<ResourceType>,
    pipeline_state: gfx::PipelineState<ResourceType, spatial::pipe::Meta>,

}

impl MeshRenderer {

    pub fn new(data: spatial::pipe::Data<ResourceType>, slice: gfx::Slice<ResourceType>, pipeline_state: gfx::PipelineState<ResourceType, spatial::pipe::Meta>) -> MeshRenderer{

        return MeshRenderer { data, slice, pipeline_state };

    }

    pub fn create(vertices: &[UvVertex3f], texture: &render::Texture, v_shader: &[u8], f_shader: &[u8], renderer: &mut core::Renderer) -> MeshRenderer {
        // Load shaders.
        let pipeline_state = renderer.factory
            .create_pipeline_simple(
                v_shader,
                f_shader,
                pipe::new(),
            )
            .unwrap();
        let (vertex_buffer, slice) = renderer.factory.create_vertex_buffer_with_slice(vertices, ());
        let trans_buffer = renderer.factory.create_constant_buffer(1);

        let sampler = renderer.factory.create_sampler_linear();

        let data = pipe::Data {
            vbuf: vertex_buffer,
            tex: (texture.get_shader_texture(renderer), sampler),
            trans: trans_buffer,
            out: renderer.render_view.clone(),
        };

        return MeshRenderer::new(data, slice, pipeline_state);

    }

    pub fn render(&mut self, model_trans: Matrix4f, view_trans: Matrix4f, projection_trans: Matrix4f, engine: &mut core::FlatEngine) {
        engine.renderer.encoder.update_buffer(&self.data.trans, &[MeshTransform { model: model_trans.get_data(), view: view_trans.get_data(), projection: projection_trans.get_data() }], 0); //update buffers
        engine.renderer.encoder.draw(&self.slice, &mut self.pipeline_state, &self.data); // draw commands with buffer data and attached pso
        engine.renderer.encoder.flush(engine.renderer.device.as_mut()); // execute draw commands
    }

    pub fn update_vertices(&mut self, vertices: &[UvVertex3f], renderer: &mut core::Renderer) {
        let (vertex_buffer, slice) = renderer.factory.create_vertex_buffer_with_slice(vertices, ());
        self.data.vbuf = vertex_buffer;
        self.slice = slice;
    }

    pub fn update_texture(&mut self, texture: &render::Texture, renderer: &mut core::Renderer) {
        let sampler = renderer.factory.create_sampler_linear();
        self.data.tex = (texture.get_shader_texture(renderer), sampler);
    }


}

pub struct Mesh {

    vertices: Vec<UvVertex3f>,

}

impl Mesh {

    pub fn new() -> Mesh {

        return Mesh { vertices: Vec::new() };

    }

    pub fn from_vertices(verts: Vec<Vector3f>) -> Mesh {

        let mut uvverts: Vec<UvVertex3f> = Vec::new();

        for v in verts.iter() {
            uvverts.push(UvVertex3f { pos: [v.x as f32, v.y as f32, v.z as f32], uv: [0.0, 0.0] });
        }

        return Mesh { vertices: uvverts };

    }

}

pub struct Entity {

    pub node: NodeObject3D,
    pub mesh: Mesh,
    pub texture: Option<Box<render::Texture>>,
    mesh_renderer: Option<MeshRenderer>,

}

impl Entity {

    pub fn new() -> Entity {

        return Entity { node: NodeObject3D::new(), mesh: Mesh::new(), texture: None, mesh_renderer: None };

    }

    pub fn from_mesh(mesh: Mesh, texture: Option<Box<render::Texture>>) -> Entity {

        return Entity { node: NodeObject3D::new(), mesh: mesh, texture: texture, mesh_renderer: None };

    }

}

impl core::Drawable for Entity {

    fn load(&mut self, engine: &mut core::FlatEngine) {
        if self.texture.is_some() {
            self.mesh_renderer = Some(MeshRenderer::create(&self.mesh.vertices, self.texture.as_ref().unwrap().as_ref(), include_bytes!("../../shaders/std_mesh_v.glsl"), include_bytes!("../../shaders/std_mesh_f.glsl"), &mut engine.renderer));
        } else {
            self.mesh_renderer = Some(MeshRenderer::create(&self.mesh.vertices, &render::Texture::new(), include_bytes!("../../shaders/std_mesh_v.glsl"), include_bytes!("../../shaders/std_mesh_f.glsl"), &mut engine.renderer));
        }
    }

    fn render(&mut self, engine: &mut core::FlatEngine) {
        if self.mesh_renderer.is_some() {
            self.mesh_renderer.as_mut().unwrap().render(self.node.get_trans(), engine.renderer.camera.view, engine.renderer.camera.projection, engine);
        }
    }

    fn destroy(&mut self, engine: &mut core::FlatEngine) {

    }

}

impl Node3D for Entity {

    fn get_node_obj_mut(&mut self) -> &mut NodeObject3D {
        return &mut self.node;
    }

    fn get_node_obj(&self) -> &NodeObject3D {
        return &self.node;
    }

}