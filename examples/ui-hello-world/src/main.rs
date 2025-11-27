use apheleia_ui::{node::{BasicNode, Node}, rootnode::RootNode};


fn main() {
    let mut root = RootNode::new();

    let node = BasicNode {
        ..Default::default()
    };

    root.add_node(node);
}
