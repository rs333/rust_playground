#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match head {
        None => {
            println!("Nothing here");
        }
        Some(head) => {
            println!("{} ", head.val);
            reverse_list(head.next);
        }
    }
    None
}

fn main() {
    let mut node1 = ListNode::new(1);
    let mut node2 = ListNode::new(2);
    let node3 = ListNode::new(3);
    let node4 = ListNode::new(4);

    node2.next = Some(Box::new(node3));
    if let Some(ref mut n3) = node2.next.as_mut() {
        n3.next = Some(Box::new(node4));
    }
    node1.next = Some(Box::new(node2));
    reverse_list(Some(Box::new(node1)));
}
