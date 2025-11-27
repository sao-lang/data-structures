#![allow(dead_code)]

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
        while let Some(ref mut node) = current.next {
            current = node;
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
}
