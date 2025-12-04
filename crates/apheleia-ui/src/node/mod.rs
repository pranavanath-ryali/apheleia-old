use crate::{FAKE_NODEID, NodeId};
use apheleia_core::{Attribute, Color, buffer::NodeBuffer, style::Style};

pub trait Node {
    fn render(&self, buf: &mut NodeBuffer);
}

#[derive(Clone, Copy)]
pub struct BasicNode {
    pub id: NodeId,

    pub x: u16,
    pub y: u16,

    pub width: u16,
    pub height: u16,
}

impl Default for BasicNode {
    fn default() -> Self {
        Self {
            id: FAKE_NODEID,
            x: 0,
            y: 0,
            width: 1,
            height: 1,
        }
    }
}
impl Node for BasicNode {
    fn render(&self, buf: &mut NodeBuffer) {
        for x in 0..self.width {
            for y in 0..self.height {
                buf.write_line(
                    x,
                    y,
                    "A",
                    Some(Style {
                        fg: Color::Blue,
                        ..Default::default()
                    }),
                );
            }
        }
    }
}
