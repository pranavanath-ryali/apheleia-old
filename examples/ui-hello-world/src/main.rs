use apheleia_ui::{node::{BasicNode, Node}, rootnode::RootNode};


fn main() {
    let mut root = RootNode::new();

    let node = BasicNode {
        x: 10,
        y: 10,

        width: 10,
        height: 3,
        ..Default::default()
    };

    root.add_node(node);
    let node = BasicNode {
        x: 20,
        y: 30,

        width: 10,
        height: 3,
        ..Default::default()
    };
    root.add_node(node);
    root.start();
}
