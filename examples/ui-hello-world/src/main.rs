use apheleia_ui::{Node, RootNode};

fn main() {
    let mut root = RootNode::new();

    let node = Node {
        ..Default::default()
    };

    root.add_node(node);
}
