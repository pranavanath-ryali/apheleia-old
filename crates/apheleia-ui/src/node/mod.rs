use apheleia_core::buffer::Buffer;
use crate::{FAKE_NODEID, NodeId};

#[derive(Clone, Copy)]
pub struct Node {
    pub id: NodeId,

    pub x: u16,
    pub y: u16,

    pub width: u16,
    pub height: u16,
}

impl Node {
    pub fn render(buf: &mut Buffer) {}
}

impl Default for Node {
    fn default() -> Self {
        Self { id: FAKE_NODEID, x: 0, y: 0, width: 1, height: 1 }
    }
}
