
use super::*;

use self::types::*;

#[derive(Copy, Clone)]
/**
The default node object contains the data necessary to handle a basic node (position and transform).
*/
pub struct NodeObject {

    pub pos: Vector2f,
    pub trans: Transform,
    pub values_changed: bool,

}

impl NodeObject {

    pub fn new() -> NodeObject {

        return NodeObject { pos: Vector2f::zero(), trans: Transform::identity(), values_changed: false }

    }

    pub fn acknowledge_values_changed(&mut self) -> bool {

        if self.values_changed {
            self.values_changed = false;
            return true;
        }
        return false;

    }

}

impl Node for NodeObject {

    fn set_pos(&mut self, pos: Vector2f) {
        self.pos = pos;
        self.values_changed = true;
    }
    fn get_pos(&self) -> Vector2f {
        return self.pos;
    }

    fn set_trans(&mut self, trans: Transform){
        self.trans = trans;
        self.values_changed = true;
    }
    fn get_trans(&self) -> Transform {
        return self.trans;
    }

}

pub trait Node {

    fn set_pos(&mut self, pos: Vector2f);
    fn get_pos(&self) -> Vector2f;

    fn set_trans(&mut self, transform: Transform);
    fn get_trans(&self) -> Transform;

}

/**
The sized node object contains the data necessary to handle a basic node with a size.
*/
pub struct SizedNodeObject {

    pub pos: Vector2f,
    pub trans: Transform,
    pub size: Vector2f,
    pub scale: Vector2f,
    pub values_changed: bool,

}

impl SizedNodeObject {

    pub fn new() -> SizedNodeObject {

        return SizedNodeObject { pos: Vector2f::zero(), trans: Transform::identity(), size: Vector2f::zero(), scale: Vector2f::new(1.0, 1.0), values_changed: false }

    }

    pub fn from_size(size: Vector2f) -> SizedNodeObject {

        return SizedNodeObject { pos: Vector2f::zero(), trans: Transform::identity(), size: size, scale: Vector2f::new(1.0, 1.0), values_changed: false }

    }

    pub fn acknowledge_values_changed(&mut self) -> bool {

        if self.values_changed {
            self.values_changed = false;
            return true;
        }
        return false;

    }

}

impl Node for SizedNodeObject {

    fn set_pos(&mut self, pos: Vector2f) {
        self.pos = pos;
        self.values_changed = true;
    }
    fn get_pos(&self) -> Vector2f {
        return self.pos;
    }

    fn set_trans(&mut self, trans: Transform){
        self.trans = trans;
        self.values_changed = true;
    }
    fn get_trans(&self) -> Transform {
        return self.trans;
    }

}

impl SizedNode for SizedNodeObject {

    fn set_size(&mut self, size: Vector2f) {
        self.size = size;
        self.values_changed = true;
    }
    fn get_size(&self) -> Vector2f {
        return self.size;
    }

    fn set_scale(&mut self, scale: Vector2f) {
        self.scale = scale;
        self.values_changed = true;
    }
    fn get_scale(&self) -> Vector2f {
        return self.scale;
    }

    fn get_scaled_size(&self) -> Vector2f {
        return self.size * self.scale;
    }

}

pub trait SizedNode : Node {

    fn set_size(&mut self, size: Vector2f);
    fn get_size(&self) -> Vector2f;

    fn set_scale(&mut self, scale: Vector2f);
    fn get_scale(&self) -> Vector2f;

    fn get_scaled_size(&self) -> Vector2f;

}