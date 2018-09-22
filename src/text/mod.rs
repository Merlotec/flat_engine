use super::*;
use self::node::*;

use std::convert::AsRef;
use std::fs::File;
use std::io::{Read, Write};
use std::time::SystemTime;
pub use rusttype::Font;

impl Texture {

    pub fn from_text(text: &str, font: &rusttype::Font, size: f32, color: Color) -> Texture {
        // The font size to use
        let scale = rusttype::Scale::uniform(size);

        // Use a dark red colour
        let color = ((color.r * (255 as f32)) as u8, (color.g * (255 as f32)) as u8, (color.b * (255 as f32)) as u8);

        let v_metrics = font.v_metrics(scale);

        // layout the glyphs in a line with 20 pixels padding
        let glyphs: Vec<_> = font
            .layout(text, scale, rusttype::point(20.0, 20.0 + v_metrics.ascent))
            .collect();

        // work out the layout size
        let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
        let glyphs_width = {
            let min_x = glyphs
                .first()
                .map(|g| g.pixel_bounding_box().unwrap().min.x)
                .unwrap();
            let max_x = glyphs
                .last()
                .map(|g| g.pixel_bounding_box().unwrap().max.x)
                .unwrap();
            (max_x - min_x) as u32
        };

        // Create a new rgba image with some padding
        let mut image = image::DynamicImage::new_rgba8(glyphs_width + 40, glyphs_height + 40).to_rgba();

        // Loop through the glyphs in the text, positing each one on a line
        for glyph in glyphs {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                // Draw the glyph into the image per-pixel by using the draw closure
                glyph.draw(|x, y, v| {
                    image.put_pixel(
                        // Offset the position by the glyph bounding box
                        x + bounding_box.min.x as u32,
                        y + bounding_box.min.y as u32,
                        // Turn the coverage into an alpha value
                        image::Rgba {
                            data: [color.0, color.1, color.2, (v * 255.0) as u8],
                        },
                    )
                });
            }
        }

        let (width, height) = image.dimensions();

        return Texture { data: Vec::from(image.as_ref()), dimensions: Vector2::new(width as u16, height as u16) };
    }

}

impl Sprite {

    pub fn from_text(text: &str, font: &rusttype::Font, size: f32, color: Color) -> Sprite {

        return Sprite {
            node: SizedNodeObject::new(),
            texture: Some(Texture::from_text(text, font, size, color)),
            vertices: UvVertexArray::zero(),
            texture_renderer: None,
            update_texture: false,
            has_loaded: false
        }

    }

}

pub struct Text<'a> {

    pub node: SizedNodeObject,
    pub vertices: UvVertexArray,
    pub text: String,
    pub texture: Texture,
    pub texture_renderer: Option<TextureRenderer>,
    pub font: Font<'a>,
    pub size: f32,
    pub color: Color,
    pub update_text: bool,
    pub has_loaded: bool,

}

impl<'a> Text<'a> {

    pub fn new(text: &'a str, font: Font<'a>, size: f32, color: Color) -> Text<'a> {

        let tex = Texture::from_text(text, &font, size, color);

        return Text {
            node: SizedNodeObject::from_size(Text::adjust_size(&tex, size)),
            vertices: UvVertexArray::zero(),
            text: text.to_string(),
            texture: tex,
            texture_renderer: None,
            font: font,
            size: size,
            color: color,
            update_text: false,
            has_loaded: false
        };

    }

    pub fn set_color(&mut self, color: Color) {

        self.color = color;

        if self.has_loaded {
            self.update_text = true;
        }

    }

    pub fn set_text(&mut self, text: String) {

        self.text = text;

        self.texture = Texture::from_text(&self.text, &self.font, self.size, self.color);

        self.node.size = Text::adjust_size(&self.texture, self.size);

        if self.has_loaded {
            self.update_text = true;
        }

    }

    fn adjust_size(tex: &Texture, size: f32) -> Vector2f {

        let mut sf: f64 = tex.dimensions.x as f64 / tex.dimensions.y as f64;

        return Vector2f::new(size as f64 * sf, size as f64);

    }

}

impl<'a> Node for Text<'a> {

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

impl<'a> SizedNode for Text<'a> {

    fn set_size(&mut self, size: Vector2f) {
        //self.node.set_size(size);
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

impl<'a> core::Drawable for Text<'a> {

    fn load(&mut self, engine: &mut core::FlatEngine) {

        self.vertices = UvVertexArray::from_sized_node(self);

        self.texture_renderer = Some(TextureRenderer::create(&self.texture, &self.vertices.data, include_bytes!("../../shaders/std_texture_v.glsl"), include_bytes!("../../shaders/std_texture_f.glsl"), &mut engine.renderer));

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

            if self.update_text {

                self.texture_renderer.as_mut().unwrap().update_texture(&self.texture, &mut engine.renderer);

                // Recreate the vertices.
                self.vertices = UvVertexArray::from_sized_node(self);
                // Submit the new vertices to the buffer.
                self.texture_renderer.as_mut().unwrap().update_vertices(&self.vertices.data, &mut engine.renderer);

                self.update_text = false;
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
