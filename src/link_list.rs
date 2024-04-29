/**
 * Your MyLinkedList object will be instantiated and called as such:
 * let obj = MyLinkedList::new();
 * let ret_1: i32 = obj.get(index);
 * obj.add_at_head(val);
 * obj.add_at_tail(val);
 * obj.add_at_index(index, val);
 * obj.delete_at_index(index);
 */
pub struct MyLinkedList {
    pub len: i32,
    pub root: Option<Box<MyLinkedNode>>,
}

pub struct MyLinkedNode {
    pub val: i32,
    pub next: Option<Box<MyLinkedNode>>,
}

impl MyLinkedNode {
    pub fn new(v: i32) -> Self {
        MyLinkedNode { val: v, next: None }
    }
}

impl Default for MyLinkedList {
    fn default() -> Self {
        MyLinkedList::new()
    }
}
impl MyLinkedList {
    pub fn new() -> Self {
        MyLinkedList { len: 0, root: None }
    }

    pub fn get(&mut self, index: i32) -> i32 {
        if self.len <= index {
            return -1;
        }

        let mut h = self.root.as_mut().unwrap();
        for _ in 0..index {
            h = h.next.as_mut().unwrap();
        }
        h.val
    }

    pub fn add_at_head(&mut self, val: i32) {
        let next = self.root.take();
        self.root = Some(Box::new(MyLinkedNode { val, next }));
        self.len += 1;
    }

    pub fn add_at_tail(&mut self, val: i32) {
        let n = Some(Box::new(MyLinkedNode { val, next: None }));

        if self.root.is_none() {
            self.root = n;
        } else {
            let mut cur = self.root.as_mut().unwrap();
            while cur.next.is_some() {
                cur = cur.next.as_mut().unwrap();
            }
            cur.next = n;
        }
        self.len += 1;
    }

    pub fn add_at_index(&mut self, index: i32, val: i32) {
        match index {
            0 => self.add_at_head(val),
            index if index > self.len => (),
            index if index == self.len => self.add_at_tail(val),
            _ => {
                let mut cur = self.root.as_mut().unwrap();
                for _ in 0..index - 1 {
                    cur = cur.next.as_mut().unwrap();
                }
                let next_node = cur.next.take().unwrap();
                cur.next = Some(Box::new(MyLinkedNode {
                    val,
                    next: Some(next_node),
                }));
                self.len += 1;
            }
        }
    }

    pub fn delete_at_index(&mut self, index: i32) {
        if index >= self.len {
            return;
        }
        match (self.len, index) {
            (1, 0) => self.root = None,
            (_, 0) => self.root = self.root.as_mut().unwrap().next.take(),
            _ => {
                let mut cur = self.root.as_mut().unwrap();
                for _ in 0..(index - 1) {
                    cur = cur.next.as_mut().unwrap();
                }
                let del_node = cur.next.as_mut().unwrap();
                let rest_nodes = del_node.next.take();
                cur.next = rest_nodes;
            }
        }
        self.len -= 1;
    }
}
