
use super::*;

use self::types::*;

#[derive(Copy, Clone)]
pub struct NodeObject {

    pub pos: Vector2f,
    pub trans: Transform,

}

impl NodeObject {

    pub fn new() -> NodeObject {

        return NodeObject { pos: Vector2f::zero(), trans: Transform::identity() }

    }

}

impl Node for NodeObject {

    fn set_pos(&mut self, pos: Vector2f) {
        self.pos = pos;
    }
    fn get_pos(&self) -> Vector2f {
        return self.pos;
    }

    fn set_trans(&mut self, trans: Transform){
        self.trans = trans;
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