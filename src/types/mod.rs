use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

pub use cgmath::*;


// Using 64 bit types for standard vectors.
// This can be changed to use 32 bit types if memory is limited.
pub type Vector2f = Vector2<f32>;
pub type Vector2i = Vector2<i32>;
pub type Vector2u = Vector2<u32>;

pub type Vector3f = Vector3<f32>;
pub type Vector3i = Vector3<i32>;
pub type Vector3u = Vector3<u32>;

pub type Vector4f = Vector4<f32>;
pub type Vector4i = Vector4<i32>;
pub type Vector4u = Vector4<u32>;

pub type Matrix4f = Matrix4<f32>;
pub type Matrix4i = Matrix4<i32>;
pub type Matrix4u = Matrix4<u32>;

pub trait ToVec3f {

    fn to_vec3(&self) -> Vector3f;

}

pub trait ToVec2f {

    fn to_vec2(&self) -> Vector2f;

}

impl ToVec3f for Vector2f {
    fn to_vec3(&self) -> Vector3f {
        return Vector3f { x: self.x, y: self.y, z: 0.0 };
    }
}

impl ToVec2f for Vector3f {
    fn to_vec2(&self) -> Vector2f {
        return Vector2f { x: self.x, y: self.y };
    }
}

#[derive(Copy, Clone)]
pub struct Color {

    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32

}

impl Color {

    pub fn new(r: f32, b: f32, g: f32, a: f32) -> Color {
        return Color { r, g, b, a }
    }
    pub fn black() -> Color {
        return Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }
    }
    pub fn white() -> Color {
        return Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }
    }
    pub fn red() -> Color {
        return Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }
    }
    pub fn green() -> Color {
        return Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 }
    }
    pub fn blue() -> Color {
        return Color { r: 0.0, g: 0.0, b: 1.0, a: 1.0 }
    }
    pub fn yellow() -> Color {
        return Color { r: 1.0, g: 1.0, b: 0.0, a: 1.0 }
    }
    pub fn zero() -> Color {
        return Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 }
    }
    pub fn invert(&self) -> Color {
        return Color { r: 1.0 - self.r, g: 1.0 - self.g, b: 1.0 - self.b, a: 1.0 };
    }
    pub fn to_raw_color(&self) -> [f32; 4] {
        return [self.r, self.g, self.b, self.a];
    }

}

#[derive(Copy, Clone)]
pub struct Rect {

    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,

}

impl Rect {

    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Rect {

        return Rect { x, y, width, height  };

    }

    pub fn intersects(&self, rect: Rect) -> bool {
        if rect.x + rect.width > self.x && rect.x < self.x + self.width {
            if rect.y + rect.height > self.y && rect.y < self.y + self.height {

                return true;
            }
        }
        return false;
    }

    pub fn get_pos(&self) -> Vector2f {
        return Vector2f::new(self.x, self.y);
    }

    pub fn get_size(&self) -> Vector2f {
        return Vector2f::new(self.width, self.height);
    }

}

pub trait Translatable {

    fn set_translation(&mut self, translation: Vector3f);

    fn get_translation(&self) -> Vector3f;

    fn get_negative_translation(&self) -> Matrix4f;

    fn set_scale(&mut self, scale: Vector3f);

    fn get_scale(&self) -> Vector3f;
}

pub trait Mat4fData {

    fn get_data(&self) -> [[f32; 4]; 4];

}

impl Mat4fData for Matrix4f {

    fn get_data(&self) -> [[f32; 4]; 4] {
        return [
            [self.x.x, self.x.y, self.x.z, self.x.w],
            [self.y.x, self.y.y, self.y.z, self.y.w],
            [self.z.x, self.z.y, self.z.z, self.z.w],
            [self.w.x, self.w.y, self.w.z, self.w.w],
        ];
    }

}

impl Translatable for Matrix4f {

    fn set_translation(&mut self, translation: Vector3f) {

        self.w.x = translation.x;
        self.w.y = translation.y;
        self.w.z = translation.z;

    }

    fn get_translation(&self) -> Vector3f {
        return Vector3f { x: self.w.x, y: self.w.y, z: self.w.z };
    }

    fn get_negative_translation(&self) -> Matrix4f {

        let mut mat: Matrix4f = Matrix4f::identity();
        let translate: Vector3f = self.get_translation();

        mat.w.x = -translate.x;
        mat.w.y = -translate.y;
        mat.w.z = -translate.z;

        return mat;

    }

    fn set_scale(&mut self, scale: Vector3f) {

        self.x.x = scale.x;
        self.y.y = scale.y;
        self.z.z = scale.z;

    }

    fn get_scale(&self) -> Vector3f {
        return Vector3f { x: self.x.x, y: self.y.y, z: self.z.z };
    }

}