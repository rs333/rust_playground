use rust_playground::link_list::MyLinkedNode as Node;
#[test]
fn new() {
    let root = Node::new(42);
    assert_eq!(root.val, 42);
}
