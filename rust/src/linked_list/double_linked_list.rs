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
        self.size += 1;
    }

    pub fn delete_at_head(&mut self) -> Option<T>
    where
        T: Copy,
    {
        // 头节点删除，直接置为None
        let head = self.head.take();
        let head_rc = match head {
            None => {
                // 直接整体返回None
                return None;
            }
            Some(node_rc) => node_rc,
        };
        let data = head_rc.borrow().data;
        // 头节点删除，next也置为None
        match head_rc.borrow_mut().next.take() {
            None => {
                self.tail = None;
            }
            // next_node_ref只是要删除的节点中取出来的对下一个节点引用中的值
            // 实际的指向还是head的下一个节点
            Some(next_node_ref) => {
                next_node_ref.borrow_mut().prev = None;
                self.head = Some(next_node_ref.clone());
            }
        }
        self.size -= 1;
        return Some(data);
    }

    pub fn delete_at_tail(&mut self) -> Option<T>
    where
        T: Copy,
    {
        let tail = self.tail.take();
        let tail_rc = match tail {
            None => return None,
            Some(node_rc) => node_rc,
        };
        let data = tail_rc.borrow().data;
        // 清空tail的next指向
        if let Some(next_node_ref) = tail_rc.borrow_mut().prev.take() {
            // next_node_ref的prev是弱引用，不用管
            self.tail = next_node_ref.upgrade();
            self.size -= 1;
            return Some(data);
        } else {
            return None;
        }
    }

    pub fn find(&mut self, data: T) -> Option<usize>
    where
        T: PartialEq + Copy,
    {
        if self.is_empty() {
            return None;
        }
        // let mut current = self.head.clone();
        // let mut index = 0;
        // loop {
        //     let node_rc = match current.take() {
        //         None => {
        //             return None;
        //         },
        //         Some(node) => node,
        //     };
        //     if node_rc.borrow().data == data {
        //         return Some(index);
        //     }
        //     current = node_rc.borrow_mut().next.clone();
        //     index += 1;
        // }
        let mut current = self.head.as_ref().unwrap().clone();
        let mut index = 0;
        loop {
            match Some(&mut current) {
                None => {
                    return None;
                },
                Some(node) => {
                    if node.borrow().data == data {
                        return Some(index);
                    }
                    current = node.clone();
                    index += 1;
                }
            }
        }
    }
}
