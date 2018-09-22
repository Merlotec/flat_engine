use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

use cgmath::Matrix4;

pub trait Vector<T> {

    fn at(self, index: usize) -> T;

}

#[derive(Copy, Clone)]
pub struct Vector2<T> {

    pub x: T,
    pub y: T

}

impl<T> Vector2<T> {

    pub fn new(x: T, y: T) -> Vector2<T> {

        return Vector2 { x, y };

    }

}

impl<T> Vector<T> for Vector2<T> {

    fn at(self, index: usize) -> T {

        if index == 0 {
            return self.x;
        }
        if index == 1 {
            return self.y;
        } else {
            panic!("Index out of range for vector.")
        }

    }

}

impl<T : Add<Output=T>> Add for Vector2<T> {

    type Output = Vector2<T>;

    fn add(self, other: Vector2<T>) -> Vector2<T> {
        return Vector2 { x: self.x + other.x, y: self.y + other.y };
    }

}

impl<T : Sub<Output=T>> Sub for Vector2<T> {

    type Output = Vector2<T>;

    fn sub(self, other: Vector2<T>) -> Vector2<T> {
        return Vector2 { x: self.x - other.x, y: self.y - other.y };
    }

}

impl<T : Mul<Output=T>> Mul for Vector2<T> {

    type Output = Vector2<T>;

    fn mul(self, other: Vector2<T>) -> Vector2<T> {
        return Vector2 { x: self.x * other.x, y: self.y * other.y };
    }

}

impl<T : Div<Output=T>> Div for Vector2<T> {

    type Output = Vector2<T>;

    fn div(self, other: Vector2<T>) -> Vector2<T> {
        return Vector2 { x: self.x / other.x, y: self.y / other.y };
    }

}

// Using 64 bit types for standard vectors.
// This can be changed to use 32 bit types if memory is limited.
pub type Vector2f = Vector2<f64>;
pub type Vector2i = Vector2<i64>;
pub type Vector2u = Vector2<u64>;

impl Vector2f {
    pub fn zero() -> Vector2f {
        return Vector2f { x: 0.0, y: 0.0 };
    }
}

impl Vector2i {
    pub fn zero() -> Vector2i {
        return Vector2i { x: 0, y: 0 };
    }
}

impl Vector2u {
    pub fn zero() -> Vector2u {
        return Vector2u { x: 0, y: 0 };
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
        return Color { r, b, g, a }
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
    pub fn zero() -> Color {
        return Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 }
    }
    pub fn invert(&self) -> Color {
        return Color { r: 1.0 - self.r, g: 1.0 - self.g, b: 1.0 - self.b, a: 1.0 };
    }
    pub fn to_raw_color(&self) -> [f32; 4] {
        return [self.r, self.b, self.g, self.a];
    }

}

#[derive(Copy, Clone)]
pub struct Transform {

    pub data: [[f32; 4]; 4]

}

impl Transform {

    pub fn identity() -> Transform {
        return Transform {
            data: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0]
            ]
        };
    }

    pub fn from_matrix(matrix: Matrix4<f32>) -> Transform {

        return Transform {
            data: [
                [matrix.x.x, matrix.y.x, matrix.z.x, matrix.w.x],
                [matrix.x.y, matrix.y.y, matrix.z.y, matrix.w.y],
                [matrix.x.z, matrix.y.z, matrix.z.z, matrix.w.z],
                [matrix.x.w, matrix.y.w, matrix.z.w, matrix.w.w]
            ]
        };

    }


}