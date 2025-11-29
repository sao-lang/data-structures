#![allow(dead_code)]

use std::fmt::Debug;

type SingleLinked<T> = Option<Box<SingleLinkedNode<T>>>;

#[derive(Debug)]
struct SingleLinkedNode<T> {
    pub data: T,
    pub next: SingleLinked<T>,
}

impl<T> SingleLinkedNode<T> {
    fn new(data: T, next: SingleLinked<T>) -> Box<Self> {
        return Box::new(SingleLinkedNode { data, next });
    }
}

#[derive(Debug)]
pub struct SingleLinkedList<T> {
    head: SingleLinked<T>,
    size: usize,
}

impl<T> SingleLinkedList<T> {
    pub fn new() -> Self {
        SingleLinkedList {
            head: None,
            size: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn insert_at_head(&mut self, data: T) {
        let mut node = SingleLinkedNode::new(data, None);
        node.next = self.head.take();
        self.head = Some(node);
        self.size += 1;
    }

    pub fn insert_at_tail(&mut self, data: T) {
        let new_node = SingleLinkedNode::new(data, None);
        if self.is_empty() {
            self.head = Some(new_node);
            self.size += 1;
            return;
        }
        let mut current = self.head.as_mut().unwrap();
        // 使用ref固定当前的引用在这一次循环里面
        // while let Some(ref mut node) = current.next {
        //     current = node;
        // }
        // current.next = Some(new_node);
        // self.size += 1;
        loop {
            match current.next {
                Some(ref mut node) => {
                    current = node;
                }
                None => {
                    break;
                }
            }
        }
        current.next = Some(new_node);
        self.size += 1;
    }

    pub fn delete_at_head(&mut self) -> Option<T> {
        self.head.take().map(|mut old_head| {
            self.head = old_head.next.take();
            self.size -= 1;
            old_head.data
        })
    }

    pub fn delete(&mut self, data: T) -> Option<T>
    where
        T: PartialEq,
    {
        if self.is_empty() {
            return None;
        }
        if self.head.as_ref().unwrap().data == data {
            return self.delete_at_head();
        }
        let mut current = self.head.as_mut();
        // loop {
        //     match current {
        //         Some(node) => {
        //             if node.data == data {
        //                 self.size -= 1;
        //                 return Some(data);
        //             }
        //             current = node.next.as_mut();
        //         },
        //         None => {
        //             return None;
        //         }
        //     }
        // }
        while let Some(node) = current {
            if node.data == data {
                self.size -= 1;
                return Some(data);
            }
            current = node.next.as_mut();
        }
        return None;
    }

    pub fn find(&mut self, data: T) -> Option<usize>
    where
        T: PartialEq,
    {
        if self.is_empty() {
            return None;
        }
        let mut current = self.head.as_mut();
        let mut index = 0;
        while let Some(node) = current {
            if node.data == data {
                return Some(index);
            }
            current = node.next.as_mut();
            index += 1;
        }
        None
        // loop {
        //     match current {
        //         None => {
        //             return None;
        //         }
        //         Some(node) if node.data == data => {
        //             return Some(index);
        //         }
        //         Some(node) => {
        //             current = node.next.as_mut();
        //             index += 1;
        //         }
        //     }
        // }
    }

    pub fn display(&mut self)
    where
        T: Debug,
    {
        let mut result = String::new();
        let mut current = self.head.as_ref();

        // loop {
        //     match current {
        //         None => {
        //             break;
        //         }
        //         Some(node) => {
        //             result = format!("{:?}", node.data);
        //             if !node.next.is_none() {
        //                 result = format!("{:?}", "->");
        //                 current = node.next.as_ref();
        //             }
        //             current = node.next.as_ref();
        //         }
        //     }
        // }

        while let Some(node) = current {
            result = format!("{:?}", node.data);
            if !node.next.is_none() {
                result = format!("{:?}", "->");
                current = node.next.as_ref();
            }
        }
        print!("{:?}", result);
    }
}
