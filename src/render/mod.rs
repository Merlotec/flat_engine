
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

        model: [[f32; 4]; 4] = "model_Transform",
        view: [[f32; 4]; 4] = "view_Transform",
        projection: [[f32; 4]; 4] = "projection_Transform",

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

    pub fn from_rect(rect: &Rect) -> UvVertexArray {

        return UvVertexArray {
            data: [
                UvVertex2f { pos: [rect.x, rect.y], uv: [0.0, 1.0] },
                UvVertex2f { pos: [rect.x + rect.width, rect.y], uv: [1.0, 1.0] },
                UvVertex2f { pos: [rect.x, rect.y + rect.height], uv: [0.0, 0.0] },
                UvVertex2f { pos: [rect.x + rect.width, rect.y + rect.height], uv: [1.0, 0.0] },
                UvVertex2f { pos: [rect.x + rect.width, rect.y], uv: [1.0, 1.0] },
                UvVertex2f { pos: [rect.x, rect.y + rect.height], uv: [0.0, 0.0] }
            ]
        }

    }

}

pub struct Texture {

    pub data: Vec<u8>,
    pub dimensions: Vector2<u16>,

}

impl Texture {

    pub fn new() -> Texture {

        return Texture { data: vec![0, 0, 0, 0], dimensions: Vector2::new(1, 1) };

    }

    pub fn from_data(data: &[u8], width: u16, height: u16) -> Texture {

        return Texture { data: Vec::from(data), dimensions: Vector2::new(width, height) };

    }

    pub fn from_image(image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>) -> Texture {

        return Texture::from_data(image.as_ref(), image.dimensions().0 as u16, image.dimensions().1 as u16);

    }

    pub fn load_from_path(path: &str) -> Texture {
        use gfx::format::Rgba8;
        let img = image::open(path).unwrap().to_rgba();
        let (width, height) = img.dimensions();
        return Texture { data: Vec::from(img.as_ref()), dimensions: Vector2::new(width as u16, height as u16) };

    }

    pub fn load_from_image(bytes: &[u8]) -> Texture {
        use gfx::format::Rgba8;
        let img = image::load_from_memory(bytes).unwrap().to_rgba();
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

    // Automatically applies global Matrix4f to the render.
    pub fn render(&mut self, model_trans: Matrix4f, view_trans: Matrix4f, projection_trans: Matrix4f, engine: &mut core::FlatEngine) {
        //println!("{}", model_trans.x.x);
        engine.renderer.encoder.update_buffer(&self.data.trans, &[GeometryTransform { model: model_trans.get_data(), view: view_trans.get_data(), projection: projection_trans.get_data() }], 0); //update buffers
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

    pub node: NodeObject2D,
    pub texture: Box<Texture>,
    pub vertices: UvVertexArray,
    pub texture_renderer: Option<TextureRenderer>,
    pub update_texture: bool,
    pub has_loaded: bool,

}

impl Sprite {

    pub fn new() -> Sprite {

        return Sprite {
            node: NodeObject2D::new(),
            texture: Box::new(Texture::new()),
            vertices: UvVertexArray::zero(),
            texture_renderer: None,
            update_texture: false,
            has_loaded: false,
        };

    }

    pub fn from_texture(texture: Box<Texture>) -> Sprite {
        return Sprite {
            node: NodeObject2D::new(),
            texture: texture,
            vertices: UvVertexArray::zero(),
            texture_renderer: None,
            update_texture: false,
            has_loaded: false,
        };
    }

    pub fn from_image_path(path: &'static str) -> Sprite {

        let texture = Texture::load_from_path(path);

        let size = Vector2f { x: texture.dimensions.x as f32, y: texture.dimensions.y as f32 };

        let mut sprite = Sprite::from_texture(Box::new(texture));

        sprite.set_size(size);

        return sprite;

    }

    pub fn set_texture(&mut self, texture: Box<Texture>) {

        self.texture = texture;

        if self.has_loaded {
            self.update_texture = true;
        }

    }

}

impl core::Drawable for Sprite {

    fn load(&mut self, engine: &mut core::FlatEngine) {
        self.vertices = UvVertexArray::from_rect(&Rect { x: 0.0, y: 0.0, width: self.get_fixed_size().x, height: self.get_fixed_size().y });
        self.texture_renderer = Some(TextureRenderer::create(self.texture.as_ref(), &self.vertices.data, include_bytes!("../../shaders/std_texture_v.glsl"), include_bytes!("../../shaders/std_texture_f.glsl"), &mut engine.renderer));
        self.has_loaded = true;

    }

    fn render(&mut self, engine: &mut core::FlatEngine) {

        // Check if all neccessary parts have been initialized.
        if self.texture_renderer.is_some() {

            // If the texture has changed, update the texture.
            if self.update_texture {

                if !self.has_loaded {
                    self.load(engine);
                } else {
                    self.texture_renderer.as_mut().unwrap().update_texture(self.texture.as_ref(), &mut engine.renderer);

                }
                self.update_texture = false;
            }

            self.texture_renderer.as_mut().unwrap().render(self.node.get_trans(), engine.renderer.camera.view, engine.renderer.camera.projection, engine);

        } else {
            // We never want to see this.
            panic!("The sprite object is being drawn before it has been initialized!");
        }

    }

    fn destroy(&mut self, engine: &mut core::FlatEngine) {

    }

}

impl Node2D for Sprite {

    fn get_node_obj_mut(&mut self) -> &mut NodeObject2D {
        return &mut self.node;
    }

    fn get_node_obj(&self) -> &NodeObject2D {
        return &self.node;
    }

}

impl SizedNode2D for Sprite {

    fn get_fixed_size(&self) -> Vector2f {
        return Vector2f { x: self.texture.as_ref().dimensions.x as f32, y: self.texture.as_ref().dimensions.y as f32 };
    }

}