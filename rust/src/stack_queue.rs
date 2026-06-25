#![allow(dead_code)]

use std::fmt;
use crate::array::{DynamicArray, ArrayError};
use crate::linked_list::{SinglyLinkedList, DoublyLinkedList};

#[derive(Debug, PartialEq, Eq)]
pub enum StackQueueError {
    CapacityMustBePositive,
    StackIsEmpty,
    QueueIsEmpty,
    DequeIsEmpty,
}

impl fmt::Display for StackQueueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StackQueueError::CapacityMustBePositive => write!(f, "capacity must be positive"),
            StackQueueError::StackIsEmpty => write!(f, "stack is empty"),
            StackQueueError::QueueIsEmpty => write!(f, "queue is empty"),
            StackQueueError::DequeIsEmpty => write!(f, "deque is empty"),
        }
    }
}

impl std::error::Error for StackQueueError {}

pub struct Stack<T: Clone + Default> {
    items: DynamicArray<T>,
}

impl<T: Clone + Default> Stack<T> {
    pub fn new() -> Self {
        Stack {
            items: DynamicArray::new(10).unwrap(),
        }
    }

    pub fn size(&self) -> usize {
        self.items.length()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn pop(&mut self) -> Result<T, ArrayError> {
        self.items.pop()
    }

    pub fn peek(&self) -> Result<&T, ArrayError> {
        if self.is_empty() {
            return Err(ArrayError::ArrayIsEmpty);
        }
        self.items.at(self.items.length() - 1)
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn to_vec(&self) -> Vec<T> {
        self.items.to_vec()
    }
}

impl<T: Clone + Default> IntoIterator for Stack<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.to_vec().into_iter()
    }
}

impl<'a, T: Clone + Default> IntoIterator for &'a Stack<T> {
    type Item = &'a T;
    type IntoIter = crate::array::DynamicArrayRefIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.items).into_iter()
    }
}

impl<T: Clone + Default + fmt::Display> fmt::Display for Stack<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let elements: Vec<String> = self.to_vec().iter().map(|x| x.to_string()).collect();
        write!(f, "[{}]", elements.join(", "))
    }
}

pub struct Queue<T: Clone + Default> {
    items: SinglyLinkedList<T>,
}

impl<T: Clone + Default> Queue<T> {
    pub fn new() -> Self {
        Queue {
            items: SinglyLinkedList::new(),
        }
    }

    pub fn size(&self) -> usize {
        self.items.length()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn enqueue(&mut self, item: T) {
        self.items.append(item);
    }

    pub fn dequeue(&mut self) -> Result<T, StackQueueError> {
        self.items.remove_first().ok_or(StackQueueError::QueueIsEmpty)
    }

    pub fn peek(&self) -> Result<&T, StackQueueError> {
        self.items.head().ok_or(StackQueueError::QueueIsEmpty)
    }

    pub fn clear(&mut self) {
        self.items = SinglyLinkedList::new();
    }

    pub fn to_vec(&self) -> Vec<T> {
        self.items.to_vec()
    }
}

impl<T: Clone + Default> IntoIterator for Queue<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.to_vec().into_iter()
    }
}

impl<'a, T: Clone + Default> IntoIterator for &'a Queue<T> {
    type Item = &'a T;
    type IntoIter = crate::linked_list::SinglyLinkedListRefIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.items).into_iter()
    }
}

impl<T: Clone + Default + fmt::Display> fmt::Display for Queue<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let elements: Vec<String> = self.to_vec().iter().map(|x| x.to_string()).collect();
        write!(f, "[{}]", elements.join(", "))
    }
}

pub struct CircularQueue<T: Clone + Default> {
    capacity: usize,
    items: Vec<Option<T>>,
    front: usize,
    rear: isize,
    size: usize,
}

impl<T: Clone + Default> CircularQueue<T> {
    pub fn new(capacity: usize) -> Result<Self, StackQueueError> {
        if capacity == 0 {
            return Err(StackQueueError::CapacityMustBePositive);
        }
        Ok(CircularQueue {
            capacity,
            items: vec![None; capacity],
            front: 0,
            rear: -1,
            size: 0,
        })
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    pub fn enqueue(&mut self, item: T) -> bool {
        if self.is_full() {
            return false;
        }
        self.rear = (self.rear + 1) % self.capacity as isize;
        self.items[self.rear as usize] = Some(item);
        self.size += 1;
        true
    }

    pub fn dequeue(&mut self) -> Result<T, StackQueueError> {
        if self.is_empty() {
            return Err(StackQueueError::QueueIsEmpty);
        }
        let item = self.items[self.front].take().ok_or(StackQueueError::QueueIsEmpty)?;
        self.front = (self.front + 1) % self.capacity;
        self.size -= 1;
        Ok(item)
    }

    pub fn peek(&self) -> Result<&T, StackQueueError> {
        if self.is_empty() {
            return Err(StackQueueError::QueueIsEmpty);
        }
        self.items[self.front].as_ref().ok_or(StackQueueError::QueueIsEmpty)
    }

    pub fn clear(&mut self) {
        self.items = vec![None; self.capacity];
        self.front = 0;
        self.rear = -1;
        self.size = 0;
    }

    pub fn to_vec(&self) -> Vec<T> {
        let mut result = Vec::with_capacity(self.size);
        for i in 0..self.size {
            let index = (self.front + i) % self.capacity;
            if let Some(item) = &self.items[index] {
                result.push(item.clone());
            }
        }
        result
    }
}

impl<T: Clone + Default> IntoIterator for CircularQueue<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.to_vec().into_iter()
    }
}

pub struct CircularQueueRefIter<'a, T: Clone + Default> {
    queue: &'a CircularQueue<T>,
    index: usize,
}

impl<'a, T: Clone + Default> Iterator for CircularQueueRefIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.queue.size {
            return None;
        }
        let real_index = (self.queue.front + self.index) % self.queue.capacity;
        let value = self.queue.items[real_index].as_ref();
        self.index += 1;
        value
    }
}

impl<'a, T: Clone + Default> IntoIterator for &'a CircularQueue<T> {
    type Item = &'a T;
    type IntoIter = CircularQueueRefIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        CircularQueueRefIter {
            queue: self,
            index: 0,
        }
    }
}

impl<T: Clone + Default + fmt::Display> fmt::Display for CircularQueue<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let elements: Vec<String> = self.to_vec().iter().map(|x| x.to_string()).collect();
        write!(f, "[{}]", elements.join(", "))
    }
}

pub struct Deque<T: Clone + Default> {
    items: DoublyLinkedList<T>,
}

impl<T: Clone + Default> Deque<T> {
    pub fn new() -> Self {
        Deque {
            items: DoublyLinkedList::new(),
        }
    }

    pub fn size(&self) -> usize {
        self.items.length()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn push_front(&mut self, item: T) {
        self.items.prepend(item);
    }

    pub fn push_back(&mut self, item: T) {
        self.items.append(item);
    }

    pub fn pop_front(&mut self) -> Result<T, StackQueueError> {
        self.items.remove_first().ok_or(StackQueueError::DequeIsEmpty)
    }

    pub fn pop_back(&mut self) -> Result<T, StackQueueError> {
        self.items.remove_last().ok_or(StackQueueError::DequeIsEmpty)
    }

    pub fn peek_front(&self) -> Result<&T, StackQueueError> {
        self.items.head().ok_or(StackQueueError::DequeIsEmpty)
    }

    pub fn peek_back(&self) -> Result<&T, StackQueueError> {
        self.items.tail().ok_or(StackQueueError::DequeIsEmpty)
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn to_vec(&self) -> Vec<T> {
        self.items.to_vec()
    }

    pub fn to_vec_reverse(&self) -> Vec<T> {
        self.items.to_vec_reverse()
    }
}

impl<T: Clone + Default> IntoIterator for Deque<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.to_vec().into_iter()
    }
}

impl<'a, T: Clone + Default> IntoIterator for &'a Deque<T> {
    type Item = &'a T;
    type IntoIter = crate::linked_list::DoublyLinkedListRefIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.items).into_iter()
    }
}

impl<T: Clone + Default + fmt::Display> fmt::Display for Deque<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let elements: Vec<String> = self.to_vec().iter().map(|x| x.to_string()).collect();
        write!(f, "[{}]", elements.join(", "))
    }
}

pub fn main() {
    println!("Stack Example:");
    let mut stack = Stack::<i32>::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    println!("Stack: {}", stack);
    println!("Iterating with for loop:");
    for val in &stack {
        print!("{} ", val);
    }
    println!();
    let popped = stack.pop().unwrap();
    println!("Popped: {}", popped);
    println!("Stack after pop: {}", stack);

    println!("\nQueue Example:");
    let mut queue = Queue::<i32>::new();
    queue.enqueue(10);
    queue.enqueue(20);
    queue.enqueue(30);
    println!("Queue: {}", queue);
    println!("Iterating with for loop:");
    for val in &queue {
        print!("{} ", val);
    }
    println!();
    let dequeued = queue.dequeue().unwrap();
    println!("Dequeued: {}", dequeued);
    println!("Queue after dequeue: {}", queue);

    println!("\nCircular Queue Example:");
    let mut cq = CircularQueue::<i32>::new(3).unwrap();
    cq.enqueue(100);
    cq.enqueue(200);
    cq.enqueue(300);
    println!("Circular Queue: {}", cq);
    println!("Is Full: {}", cq.is_full());
    println!("Iterating with for loop:");
    for val in &cq {
        print!("{} ", val);
    }
    println!();

    println!("\nDeque Example:");
    let mut deque = Deque::<i32>::new();
    deque.push_back(10);
    deque.push_back(20);
    deque.push_front(5);
    println!("Deque: {}", deque);
    println!("Size: {}", deque.size());
    println!("Front: {}", deque.peek_front().unwrap());
    println!("Back: {}", deque.peek_back().unwrap());
    println!("Iterating with for loop:");
    for val in &deque {
        print!("{} ", val);
    }
    println!();
    
    let popped_front = deque.pop_front().unwrap();
    println!("Popped front: {}", popped_front);
    println!("Deque after pop front: {}", deque);
    
    let popped_back = deque.pop_back().unwrap();
    println!("Popped back: {}", popped_back);
    println!("Deque after pop back: {}", deque);
    
    deque.push_front(1);
    deque.push_back(30);
    println!("Deque after pushing: {}", deque);
    println!("Reverse: {:?}", deque.to_vec_reverse());
}
