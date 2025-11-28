#![allow(dead_code)]

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};
type DoubleLinked<T> = Option<Rc<RefCell<DoubleLinkedNode<T>>>>;
type DoubleWeakLinked<T> = Option<Weak<RefCell<DoubleLinkedNode<T>>>>;
struct DoubleLinkedNode<T> {
    pub data: T,
    pub next: DoubleLinked<T>,
    pub prev: DoubleWeakLinked<T>,
}

impl<T> DoubleLinkedNode<T> {
    pub fn new(data: T, next: DoubleLinked<T>, prev: DoubleWeakLinked<T>) -> Rc<RefCell<Self>> {
        return Rc::new(RefCell::new(DoubleLinkedNode { data, prev, next }));
    }
}

struct DoubleLinkedList<T> {
    size: usize,
    head: DoubleLinked<T>,
    tail: DoubleLinked<T>,
}

impl<T> DoubleLinkedList<T> {
    pub fn new() -> Self {
        return DoubleLinkedList {
            size: 0,
            head: None,
            tail: None,
        };
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    fn initialize_list(&mut self, node: Rc<RefCell<DoubleLinkedNode<T>>>) {
        self.head = Some(node.clone());
        self.tail = Some(node.clone());
        self.size += 1;
    }

    pub fn insert_at_head(&mut self, data: T) {
        let node = DoubleLinkedNode::new(data, None, None);
        if self.is_empty() {
            self.initialize_list(node);
            return;
        }
        let head = self.head.as_mut().unwrap();
        node.borrow_mut().prev = Some(Rc::downgrade(head));
        head.borrow_mut().next = Some(node.clone());
        self.head = Some(node.clone());
        self.size += 1;
    }

    pub fn insert_at_tail(&mut self, data: T) {
        let node = DoubleLinkedNode::new(data, None, None);
        if self.is_empty() {
            self.initialize_list(node);
            return;
        }
        let tail = self.tail.as_mut().unwrap();
        tail.borrow_mut().next = Some(node.clone());
        node.borrow_mut().prev = Some(Rc::downgrade(&tail));
        self.tail = Some(node.clone());
    }
}
