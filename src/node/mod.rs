
use super::*;

use self::types::*;
use cgmath::Matrix;

#[derive(Copy, Clone)]
/**
The default node object contains the data necessary to handle a basic node (position and transform).
*/
pub struct NodeObject2D {

    pub trans: Matrix4f,

}

impl NodeObject2D {

    pub fn new() -> NodeObject2D {

        return NodeObject2D { trans: Matrix4f::identity() };

    }

    pub fn from(trans: Matrix4f) -> NodeObject2D {

        return NodeObject2D { trans: Matrix4f::identity()  };

    }

    pub fn pos_and_scale(pos: Vector2f, scale: Vector2f) -> NodeObject2D {

        let mut trans: Matrix4f = Matrix4f::identity();
        trans.set_translation(pos.to_vec3());
        trans.set_scale(scale.to_vec3());

        return NodeObject2D { trans: Matrix4f::identity()  };

    }

}

impl NodeObject2D {

    pub fn set_pos(&mut self, pos: Vector2f) {
        self.trans.set_translation(pos.to_vec3());
    }
    pub fn get_pos(&self) -> Vector2f {
        return self.trans.get_translation().to_vec2();
    }
    pub fn set_scale(&mut self, scale: Vector2f) {
        self.trans.set_scale(scale.to_vec3());
    }
    pub fn get_scale(&self) -> Vector2f {
        return self.trans.get_scale().to_vec2();
    }
    pub fn set_trans(&mut self, trans: Matrix4f) {
        self.trans = trans;
    }
    pub fn get_trans(&self) -> Matrix4f {
        return self.trans;
    }

}

pub trait Node2D {

    fn get_node_obj_mut(&mut self) -> &mut NodeObject2D;
    fn get_node_obj(&self) -> &NodeObject2D;

    fn set_pos(&mut self, pos: Vector2f) {
        self.get_node_obj_mut().set_pos(pos);
    }

    fn get_pos(&self) -> Vector2f {
        return self.get_node_obj().get_pos();
    }

    fn set_scale(&mut self, scale: Vector2f) {
        self.get_node_obj_mut().set_scale(scale);
    }
    fn get_scale(&self) -> Vector2f {
        return self.get_node_obj().get_scale();
    }
    fn set_trans(&mut self, trans: Matrix4f) {
        self.get_node_obj_mut().set_trans(trans);
    }
    fn get_trans(&self) -> Matrix4f {
        return self.get_node_obj().get_trans();
    }

}

pub trait SizedNode2D : Node2D {

    fn get_fixed_size(&self) -> Vector2f;

    fn set_size(&mut self, size: Vector2f) {

        let fs: Vector2f = self.get_fixed_size();
        if fs.x == 0.0 || fs.y == 0.0 {
            panic!("Cannot set the scaled size of an object with a fixed size of 0.")
        } else {
            self.set_scale(Vector2f { x: size.x / fs.x, y: size.y / fs.y });
        }

    }

    fn get_size(&self) -> Vector2f {
        return Vector2f { x: self.get_fixed_size().x * self.get_scale().x, y: self.get_fixed_size().y * self.get_scale().y };
    }

    fn get_rect(&self) -> Rect {

        return Rect { x: self.get_pos().x, y: self.get_pos().y, width: self.get_size().x, height: self.get_size().y };

    }

}


pub trait Node3D {

    fn get_node_obj_mut(&mut self) -> &mut NodeObject3D;

    fn get_node_obj(&self) -> &NodeObject3D;

    fn set_pos(&mut self, pos: Vector3f) {
        self.get_node_obj_mut().set_pos(pos);
    }

    fn get_pos(&self) -> Vector3f {
        return self.get_node_obj().get_pos();
    }

    fn set_scale(&mut self, scale: Vector3f) {
        self.get_node_obj_mut().set_scale(scale);
    }

    fn get_scale(&self) -> Vector3f {
        return self.get_node_obj().get_scale();
    }
    fn set_trans(&mut self, trans: Matrix4f) {
        self.get_node_obj_mut().set_trans(trans);
    }
    fn get_trans(&self) -> Matrix4f {
        return self.get_node_obj().get_trans();
    }

}

pub struct NodeObject3D {

    pub trans: Matrix4f
}

impl NodeObject3D {

    pub fn new() -> NodeObject3D {

        return NodeObject3D { trans: Matrix4f::identity() };

    }

}

impl NodeObject3D {

    pub fn set_pos(&mut self, pos: Vector3f) {
        self.trans.set_translation(pos);
    }

    pub fn get_pos(&self) -> Vector3f {
        return self.trans.get_translation();
    }

    pub fn set_scale(&mut self, scale: Vector3f) {
        self.trans.set_scale(scale);
    }

    pub fn get_scale(&self) -> Vector3f {
        return self.trans.get_scale();
    }

    pub fn set_trans(&mut self, trans: Matrix4f) {
        self.trans = trans;
    }

    pub fn get_trans(&self) -> Matrix4f {
        return self.trans;
    }

}