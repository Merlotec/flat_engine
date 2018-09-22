
use super::*;

use node::*;
use gfx::handle::ShaderResourceView;
use image;
use geometry::GeometryRenderer;
use gfx::Factory;
use gfx::traits::FactoryExt;

gfx_defines!{

    vertex UvVertex2f {
        pos: [f32; 2] = "a_Pos",
        uv: [f32; 2] = "a_Uv",
    }

    constant GeometryTransform {

        local: [[f32; 4]; 4] = "local_Transform",
        global: [[f32; 4]; 4] = "global_Transform",

    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<UvVertex2f> = (),
        tex: gfx::TextureSampler<[f32; 4]> = "t_Texture",
        trans: gfx::ConstantBuffer<GeometryTransform> = "Transform",
        out: gfx::BlendTarget<ColorFormat> = ("Target0", gfx::state::ColorMask::all(), gfx::preset::blend::ALPHA),
    }
}

impl UvVertex2f {

    pub fn zero() -> UvVertex2f {

        return UvVertex2f { pos: [0.0, 0.0], uv: [0.0, 0.0] };

    }

    pub fn print(&self) {

        println!("pos_x: {}, pos_y: {}, uv_x: {}, uv_y: {}", self.pos[0], self.pos[1], self.uv[0], self.uv[1]);

    }

}

pub struct UvVertexArray {

    pub data: [UvVertex2f; 6],

}

impl UvVertexArray {

    pub fn zero() -> UvVertexArray {
        return UvVertexArray {
            data: [UvVertex2f::zero(), UvVertex2f::zero(), UvVertex2f::zero(), UvVertex2f::zero(), UvVertex2f::zero(), UvVertex2f::zero()]
        }
    }

    pub fn from_sized_node(node: &SizedNode) -> UvVertexArray {

        return UvVertexArray {
            data: [
                UvVertex2f { pos: [node.get_pos().x as f32, node.get_pos().y as f32], uv: [0.0, 1.0] },
                UvVertex2f { pos: [node.get_pos().x as f32 + node.get_scaled_size().x as f32, node.get_pos().y as f32], uv: [1.0, 1.0] },
                UvVertex2f { pos: [node.get_pos().x as f32, node.get_pos().y as f32 + node.get_scaled_size().y as f32], uv: [0.0, 0.0] },
                UvVertex2f { pos: [node.get_pos().x as f32 + node.get_scaled_size().x as f32, node.get_pos().y as f32 + node.get_scaled_size().y as f32], uv: [1.0, 0.0] },
                UvVertex2f { pos: [node.get_pos().x as f32 + node.get_scaled_size().x as f32, node.get_pos().y as f32], uv: [1.0, 1.0] },
                UvVertex2f { pos: [node.get_pos().x as f32, node.get_pos().y as f32 + node.get_scaled_size().y as f32], uv: [0.0, 0.0] }
            ]
        }

    }

}

pub struct Texture {

    pub data: Vec<u8>,
    pub dimensions: Vector2<u16>,

}

impl Texture {

    pub fn from_data(data: &[u8], width: u16, height: u16) -> Texture {

        return Texture { data: Vec::from(data), dimensions: Vector2::new(width, height) };

    }

    pub fn load_from_path(path: &str) -> Texture {
        use gfx::format::Rgba8;
        let img = image::open(path).unwrap().to_rgba();
        let (width, height) = img.dimensions();
        return Texture { data: Vec::from(img.as_ref()), dimensions: Vector2::new(width as u16, height as u16) };

    }

    pub fn get_shader_texture(&self, renderer: &mut core::Renderer) -> gfx::handle::ShaderResourceView<ResourceType, [f32; 4]> {
        let kind = gfx::texture::Kind::D2(self.dimensions.x, self.dimensions.y, gfx::texture::AaMode::Single);
        let (_, view) = renderer.factory.create_texture_immutable_u8::<gfx::format::Rgba8>(kind, gfx::texture::Mipmap::Provided, &[self.data.as_ref()]).unwrap();
        return view;
    }

}

pub struct TextureRenderer {

    data: render::pipe::Data<ResourceType>,
    slice: gfx::Slice<ResourceType>,
    pipeline_state: gfx::PipelineState<ResourceType, render::pipe::Meta>,

}

impl TextureRenderer {

    pub fn new(data: render::pipe::Data<ResourceType>, slice: gfx::Slice<ResourceType>, pipeline_state: gfx::PipelineState<ResourceType, render::pipe::Meta>) -> TextureRenderer {
        return TextureRenderer { data, slice, pipeline_state };
    }

    pub fn create(texture: &Texture, vertices: &[UvVertex2f], v_shader: &[u8], f_shader: &[u8], renderer: &mut core::Renderer) -> TextureRenderer {
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

        return TextureRenderer::new(data, slice, pipeline_state);

    }

    // Automatically applies global transform to the render.
    pub fn render(&mut self, transform: Transform, engine: &mut core::FlatEngine) {
        engine.renderer.encoder.update_buffer(&self.data.trans, &[GeometryTransform { local: transform.data, global: engine.hints.global_trans.data }], 0); //update buffers
        engine.renderer.encoder.draw(&self.slice, &mut self.pipeline_state, &self.data); // draw commands with buffer data and attached pso
        engine.renderer.encoder.flush(engine.renderer.device.as_mut()); // execute draw commands
    }

    pub fn update_vertices(&mut self, vertices: &[UvVertex2f], renderer: &mut core::Renderer) {
        let (vertex_buffer, slice) = renderer.factory.create_vertex_buffer_with_slice(vertices, ());
        self.data.vbuf = vertex_buffer;
        self.slice = slice;
    }

    pub fn update_texture(&mut self, texture: &Texture, renderer: &mut core::Renderer) {
        let sampler = renderer.factory.create_sampler_linear();
        self.data.tex = (texture.get_shader_texture(renderer), sampler);
    }

}

pub struct Sprite {

    pub node: SizedNodeObject,
    pub texture: Option<Texture>,
    pub vertices: UvVertexArray,
    pub texture_renderer: Option<TextureRenderer>,
    pub update_texture: bool,
    pub has_loaded: bool,

}

impl Sprite {

    pub fn new(texture: Option<Texture>) -> Sprite {

        return Sprite { node: SizedNodeObject::new(), texture: texture,
            vertices: UvVertexArray::zero(),
            texture_renderer: None,
            update_texture: false,
            has_loaded: false,
        };

    }

    pub fn set_texture(&mut self, texture: Texture) {

        self.texture = Some(texture);

        if self.has_loaded {
            self.update_texture = true;
        }

    }

}

impl core::Drawable for Sprite {

    fn load(&mut self, engine: &mut core::FlatEngine) {

        self.vertices = UvVertexArray::from_sized_node(self);

        self.texture_renderer = Some(TextureRenderer::create(self.texture.as_ref().unwrap(), &self.vertices.data, include_bytes!("../../shaders/std_texture_v.glsl"), include_bytes!("../../shaders/std_texture_f.glsl"), &mut engine.renderer));

        self.has_loaded = true;

    }

    fn render(&mut self, engine: &mut core::FlatEngine) {

        // Check if all neccessary parts have been initialized.
        if self.texture_renderer.is_some() {

            // If any of the position or size values have changed, we need to update the vertices.
            if self.node.acknowledge_values_changed() {
                // Recreate the vertices.
                self.vertices = UvVertexArray::from_sized_node(self);
                // Submit the new vertices to the buffer.
                self.texture_renderer.as_mut().unwrap().update_vertices(&self.vertices.data, &mut engine.renderer);

            }

            // If the texture has changed, update the texture.
            if self.update_texture {

                self.texture_renderer.as_mut().unwrap().update_texture(self.texture.as_ref().unwrap(), &mut engine.renderer);

                self.update_texture = false;
            }

            self.texture_renderer.as_mut().unwrap().render(self.node.trans, engine);

        } else {
            // We never want to see this.
            panic!("The sprite object is being drawn before it has been initialized!");
        }

    }

    fn destroy(&mut self, engine: &mut core::FlatEngine) {

    }

}

impl Node for Sprite {

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

impl SizedNode for Sprite {

    fn set_size(&mut self, size: Vector2f) {
        self.node.set_size(size);
    }
    fn get_size(&self) -> Vector2f {
        return self.node.get_size();
    }

    fn set_scale(&mut self, scale: Vector2f) {
        self.node.set_scale(scale);
    }
    fn get_scale(&self) -> Vector2f {
        return self.node.get_scale();
    }

    fn get_scaled_size(&self) -> Vector2f {
        return self.node.get_scaled_size();
    }

}