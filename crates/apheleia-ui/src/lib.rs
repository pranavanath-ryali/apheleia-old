use std::collections::{HashMap, VecDeque};

use apheleia_core::{buffer::Buffer, renderer::Renderer, terminal};

pub type NodeId = usize;

pub const FAKE_NODEID: NodeId = 0;
pub const MAX_NODES: NodeId = 1000;

// TODO: Implement Terminal. Make these private
pub struct RootNode {
    width: u16,
    height: u16,

    available_nodeIds: VecDeque<NodeId>,
    nodes: HashMap<NodeId, Node>,

    main_buffer: Buffer,
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

            main_buffer: Buffer::new(size.0, size.1),
            renderer: Renderer::new(),
        }
    }

    fn get_id(&mut self) -> Option<NodeId> {
        self.available_nodeIds.pop_front()
    }

    pub fn add_node(&mut self, node: Node) {
        let mut new_node = node.clone();
        if new_node.id == FAKE_NODEID {
            let id = self.get_id();
            if let Some(id) = id {
                new_node.id = id;
            } else {
                return
            }
        }

        self.nodes.insert(new_node.id, new_node);
    }
}

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
