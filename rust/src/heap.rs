#![allow(dead_code)]
#![allow(unused_assignments)]

use std::fmt;
use std::cmp::PartialOrd;

pub struct MinHeap<T: Clone + PartialOrd> {
    heap: Vec<T>,
}

impl<T: Clone + PartialOrd + fmt::Display> MinHeap<T> {
    pub fn new() -> Self {
        MinHeap { heap: Vec::new() }
    }

    pub fn size(&self) -> usize {
        self.heap.len()
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    fn get_parent_index(&self, index: usize) -> usize {
        (index - 1) / 2
    }

    fn get_left_child_index(&self, index: usize) -> usize {
        2 * index + 1
    }

    fn get_right_child_index(&self, index: usize) -> usize {
        2 * index + 2
    }

    fn has_parent(&self, index: usize) -> bool {
        self.get_parent_index(index) < self.heap.len()
    }

    fn has_left_child(&self, index: usize) -> bool {
        self.get_left_child_index(index) < self.heap.len()
    }

    fn has_right_child(&self, index: usize) -> bool {
        self.get_right_child_index(index) < self.heap.len()
    }

    fn parent(&self, index: usize) -> &T {
        &self.heap[self.get_parent_index(index)]
    }

    fn left_child(&self, index: usize) -> &T {
        &self.heap[self.get_left_child_index(index)]
    }

    fn right_child(&self, index: usize) -> &T {
        &self.heap[self.get_right_child_index(index)]
    }

    fn swap(&mut self, index_one: usize, index_two: usize) {
        self.heap.swap(index_one, index_two);
    }

    fn heapify_up(&mut self) {
        let mut index = self.heap.len() - 1;
        while self.has_parent(index) && self.parent(index) > &self.heap[index] {
            let parent_index = self.get_parent_index(index);
            self.swap(parent_index, index);
            index = parent_index;
        }
    }

    fn heapify_down(&mut self) {
        let mut index = 0;
        while self.has_left_child(index) {
            let mut smaller_child_index = self.get_left_child_index(index);
            if self.has_right_child(index) && self.right_child(index) < self.left_child(index) {
                smaller_child_index = self.get_right_child_index(index);
            }

            if self.heap[index] < self.heap[smaller_child_index] {
                break;
            } else {
                self.swap(index, smaller_child_index);
            }
            index = smaller_child_index;
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.heap.first()
    }

    pub fn poll(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let item = self.heap[0].clone();
        let last_item = self.heap.pop().unwrap();
        if !self.is_empty() {
            self.heap[0] = last_item;
            self.heapify_down();
        }
        Some(item)
    }

    pub fn add(&mut self, item: T) {
        self.heap.push(item);
        self.heapify_up();
    }

    pub fn to_vec(&self) -> Vec<T> {
        self.heap.clone()
    }

    pub fn clear(&mut self) {
        self.heap.clear();
    }
}

impl<T: Clone + PartialOrd + fmt::Display> fmt::Display for MinHeap<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let elements: Vec<String> = self.heap.iter().map(|x| x.to_string()).collect();
        write!(f, "[{}]", elements.join(", "))
    }
}

pub struct MaxHeap<T: Clone + PartialOrd> {
    heap: Vec<T>,
}

impl<T: Clone + PartialOrd + fmt::Display> MaxHeap<T> {
    pub fn new() -> Self {
        MaxHeap { heap: Vec::new() }
    }

    pub fn size(&self) -> usize {
        self.heap.len()
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    fn get_parent_index(&self, index: usize) -> usize {
        (index - 1) / 2
    }

    fn get_left_child_index(&self, index: usize) -> usize {
        2 * index + 1
    }

    fn get_right_child_index(&self, index: usize) -> usize {
        2 * index + 2
    }

    fn has_parent(&self, index: usize) -> bool {
        self.get_parent_index(index) < self.heap.len()
    }

    fn has_left_child(&self, index: usize) -> bool {
        self.get_left_child_index(index) < self.heap.len()
    }

    fn has_right_child(&self, index: usize) -> bool {
        self.get_right_child_index(index) < self.heap.len()
    }

    fn parent(&self, index: usize) -> &T {
        &self.heap[self.get_parent_index(index)]
    }

    fn left_child(&self, index: usize) -> &T {
        &self.heap[self.get_left_child_index(index)]
    }

    fn right_child(&self, index: usize) -> &T {
        &self.heap[self.get_right_child_index(index)]
    }

    fn swap(&mut self, index_one: usize, index_two: usize) {
        self.heap.swap(index_one, index_two);
    }

    fn heapify_up(&mut self) {
        let mut index = self.heap.len() - 1;
        while self.has_parent(index) && self.parent(index) < &self.heap[index] {
            let parent_index = self.get_parent_index(index);
            self.swap(parent_index, index);
            index = parent_index;
        }
    }

    fn heapify_down(&mut self) {
        let mut index = 0;
        while self.has_left_child(index) {
            let mut larger_child_index = self.get_left_child_index(index);
            if self.has_right_child(index) && self.right_child(index) > self.left_child(index) {
                larger_child_index = self.get_right_child_index(index);
            }

            if self.heap[index] > self.heap[larger_child_index] {
                break;
            } else {
                self.swap(index, larger_child_index);
            }
            index = larger_child_index;
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.heap.first()
    }

    pub fn poll(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let item = self.heap[0].clone();
        let last_item = self.heap.pop().unwrap();
        if !self.is_empty() {
            self.heap[0] = last_item;
            self.heapify_down();
        }
        Some(item)
    }

    pub fn add(&mut self, item: T) {
        self.heap.push(item);
        self.heapify_up();
    }

    pub fn to_vec(&self) -> Vec<T> {
        self.heap.clone()
    }

    pub fn clear(&mut self) {
        self.heap.clear();
    }
}

impl<T: Clone + PartialOrd + fmt::Display> fmt::Display for MaxHeap<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let elements: Vec<String> = self.heap.iter().map(|x| x.to_string()).collect();
        write!(f, "[{}]", elements.join(", "))
    }
}

fn main() {
    println!("Min Heap Example:");
    let mut min_heap = MinHeap::<i32>::new();
    min_heap.add(5);
    min_heap.add(3);
    min_heap.add(7);
    min_heap.add(1);
    println!("Min Heap: {}", min_heap);
    if let Some(peek) = min_heap.peek() {
        println!("Peek: {}", peek);
    }
    if let Some(polled) = min_heap.poll() {
        println!("Polled: {}", polled);
    }
    println!("Min Heap after poll: {}", min_heap);

    println!("\nMax Heap Example:");
    let mut max_heap = MaxHeap::<i32>::new();
    max_heap.add(5);
    max_heap.add(3);
    max_heap.add(7);
    max_heap.add(1);
    println!("Max Heap: {}", max_heap);
    if let Some(peek) = max_heap.peek() {
        println!("Peek: {}", peek);
    }
    if let Some(polled) = max_heap.poll() {
        println!("Polled: {}", polled);
    }
    println!("Max Heap after poll: {}", max_heap);
}
