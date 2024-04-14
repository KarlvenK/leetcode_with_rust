use std::{cell::RefCell, rc::Rc};

struct MyHashSet {
    nodes: Vec<Option<Rc<RefCell<ListNode>>>>,
}

struct ListNode {
    val: i32,
    next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    fn new(v: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(ListNode { val: v, next: None }))
    }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {
    fn new() -> Self {
        Self {
            nodes: vec![None; 10017],
        }
    }

    fn hash(key: i32) -> usize {
        key as usize % 10016
    }

    fn add(&mut self, key: i32) {
        let pos = Self::hash(key);
        let (mut cur, mut pre) = (self.nodes[pos].clone(), None);
        while let Some(node) = cur {
            if node.borrow().val == key {
                return;
            }
            pre = Some(node.clone());
            cur = node.borrow().next.clone();
        }
        match self.nodes[pos] {
            None => self.nodes[pos] = Some(ListNode::new(key)),
            Some(_) => pre.unwrap().borrow_mut().next = Some(ListNode::new(key)),
        }
    }

    fn remove(&mut self, key: i32) {
        let pos = Self::hash(key);
        let (mut cur, mut pre) = (self.nodes[pos].clone(), None::<Rc<RefCell<ListNode>>>);
        while let Some(node) = cur {
            if node.borrow().val == key {
                if let Some(p) = pre {
                    p.borrow_mut().next = node.borrow_mut().next.take();
                } else {
                    self.nodes[pos] = node.borrow_mut().next.take();
                }
                return;
            }
            pre = Some(node.clone());
            cur = node.borrow().next.clone();
        }
    }

    fn contains(&self, key: i32) -> bool {
        let pos = Self::hash(key);
        let mut cur = self.nodes[pos].clone();
        while let Some(node) = cur {
            if node.borrow().val == key {
                return true;
            }
            cur = node.borrow().next.clone();
        }
        false
    }
}
