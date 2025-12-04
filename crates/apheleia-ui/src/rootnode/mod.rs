use std::{collections::{HashMap, VecDeque}, sync::Arc};

use crate::{
    FAKE_NODEID, MAX_NODES, NodeId,
    node::{BasicNode, Node},
};
use apheleia_core::{buffer::{Buffer, NodeBuffer}, renderer::Renderer, terminal};

// TODO: Implement Terminal. Make these private
pub struct RootNode {
    width: u16,
    height: u16,

    available_nodeIds: VecDeque<NodeId>,
    nodes: HashMap<NodeId, BasicNode>, // TODO: Find a viable solution to store Node trait rather than
                                       // just Node

    buffer: Buffer,
    renderer: Renderer,
}
impl RootNode {
    pub fn new() -> Self {
        let size = terminal::size().unwrap();

        let mut available_nodeIds: VecDeque<NodeId> = VecDeque::new();
        for i in 1..MAX_NODES {
            available_nodeIds.push_back(i);
        }

        Self {
            width: size.0,
            height: size.1,

            available_nodeIds,
            nodes: HashMap::new(),

            buffer: Buffer::new(size.0, size.1),
            renderer: Renderer::new(),
        }
    }

    fn get_id(&mut self) -> Option<NodeId> {
        self.available_nodeIds.pop_front()
    }

    pub fn add_node(&mut self, node: BasicNode) {
        if let Some(id) = self.get_id() {
            self.nodes.insert(
                id,
                node.clone()
            );
        }
    }

    pub fn start(&mut self) {
        for (id, node) in self.nodes.iter_mut() {
            let mut node_buffer = NodeBuffer::new(node.width, node.height);
            node.render(&mut node_buffer);
            self.buffer.render_node_buffer(node.x, node.y, &node_buffer);
        }

        self.renderer.flip(&mut self.buffer);
    }
}
