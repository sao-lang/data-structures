#![allow(dead_code)]

use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use std::rc::Weak;

// 单向链表节点
struct ListNode<T: Clone> {
    value: T,
    next: Option<Box<ListNode<T>>>,
}

pub struct SinglyLinkedList<T: Clone> {
    head: Option<Box<ListNode<T>>>,
    length: usize,
}

impl<T: Clone> SinglyLinkedList<T> {
    pub fn new() -> Self {
        SinglyLinkedList {
            head: None,
            length: 0,
        }
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn prepend(&mut self, value: T) {
        let new_node = Box::new(ListNode { value, next: self.head.take() });
        self.head = Some(new_node);
        self.length += 1;
    }

    pub fn append(&mut self, value: T) {
        let new_node = Box::new(ListNode { value, next: None });
        if self.head.is_none() {
            self.head = Some(new_node);
        } else {
            // 遍历到尾部再插入，避免维护尾指针与不安全代码
            let mut curr = self.head.as_mut().unwrap();
            while curr.next.is_some() {
                curr = curr.next.as_mut().unwrap();
            }
            curr.next = Some(new_node);
        }
        self.length += 1;
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    pub fn remove_first(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            let value = node.value.clone();
            self.head = node.next;
            self.length -= 1;
            value
        })
    }

    pub fn clear(&mut self) {
        self.head = None;
        self.length = 0;
    }

    pub fn to_vec(&self) -> Vec<T> {
        let mut result = Vec::new();
        let mut current = &self.head;
        while let Some(node) = current {
            result.push(node.value.clone());
            current = &node.next;
        }
        result
    }
}

pub struct SinglyLinkedListIter<T: Clone> {
    current: Option<*const ListNode<T>>,
}

impl<T: Clone> Iterator for SinglyLinkedListIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node_ptr| unsafe {
            let node = &*node_ptr;
            self.current = node.next.as_ref().map(|next| &**next as *const _);
            node.value.clone()
        })
    }
}

impl<T: Clone> IntoIterator for SinglyLinkedList<T> {
    type Item = T;
    type IntoIter = SinglyLinkedListIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        SinglyLinkedListIter {
            current: self.head.as_ref().map(|node| &**node as *const _),
        }
    }
}

impl<'a, T: Clone> IntoIterator for &'a SinglyLinkedList<T> {
    type Item = &'a T;
    type IntoIter = SinglyLinkedListRefIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        SinglyLinkedListRefIter {
            current: self.head.as_deref(),
        }
    }
}

pub struct SinglyLinkedListRefIter<'a, T: Clone> {
    current: Option<&'a ListNode<T>>,
}

impl<'a, T: Clone> Iterator for SinglyLinkedListRefIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node| {
            self.current = node.next.as_deref();
            &node.value
        })
    }
}

impl<T: Clone + fmt::Display> fmt::Display for SinglyLinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let elements: Vec<String> = self.to_vec().iter().map(|x| x.to_string()).collect();
        write!(f, "[{}]", elements.join(", "))
    }
}

// 单向循环链表节点
struct CircularListNode<T: Clone> {
    value: T,
    next: Option<Rc<RefCell<CircularListNode<T>>>>,
}

pub struct CircularLinkedList<T: Clone> {
    head: Option<Rc<RefCell<CircularListNode<T>>>>,
    length: usize,
}

impl<T: Clone> CircularLinkedList<T> {
    pub fn new() -> Self {
        CircularLinkedList {
            head: None,
            length: 0,
        }
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn head(&self) -> Option<T> {
        self.head.as_ref().map(|node| node.borrow().value.clone())
    }

    pub fn tail(&self) -> Option<T> {
        self.head.as_ref().and_then(|head| {
            let mut current = head.clone();
            loop {
                let next = current.borrow().next.clone();
                if let Some(next_node) = next {
                    if Rc::ptr_eq(&next_node, head) {
                        return Some(current.borrow().value.clone());
                    }
                    current = next_node;
                } else {
                    break;
                }
            }
            None
        })
    }

    pub fn prepend(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(CircularListNode {
            value,
            next: None,
        }));

        if let Some(head) = self.head.take() {
            let mut tail = head.clone();
            loop {
                let next = tail.borrow().next.clone();
                if let Some(next_node) = next {
                    if Rc::ptr_eq(&next_node, &head) {
                        break;
                    }
                    tail = next_node;
                } else {
                    break;
                }
            }
            new_node.borrow_mut().next = Some(head.clone());
            tail.borrow_mut().next = Some(new_node.clone());
            self.head = Some(new_node);
        } else {
            new_node.borrow_mut().next = Some(new_node.clone());
            self.head = Some(new_node);
        }
        self.length += 1;
    }

    pub fn append(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(CircularListNode {
            value,
            next: None,
        }));

        if let Some(head) = self.head.clone() {
            let mut tail = head.clone();
            loop {
                let next = tail.borrow().next.clone();
                if let Some(next_node) = next {
                    if Rc::ptr_eq(&next_node, &head) {
                        break;
                    }
                    tail = next_node;
                } else {
                    break;
                }
            }
            new_node.borrow_mut().next = Some(head);
            tail.borrow_mut().next = Some(new_node);
        } else {
            new_node.borrow_mut().next = Some(new_node.clone());
            self.head = Some(new_node);
        }
        self.length += 1;
    }

    pub fn remove_first(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }

        let old_head = self.head.take().unwrap();
        let value = old_head.borrow().value.clone();

        if self.length == 1 {
            self.length = 0;
            return Some(value);
        }

        let new_head = old_head.borrow().next.clone().unwrap();
        let mut tail = new_head.clone();
        loop {
            let next = tail.borrow().next.clone();
            if let Some(next_node) = next {
                if Rc::ptr_eq(&next_node, &old_head) {
                    break;
                }
                tail = next_node;
            } else {
                break;
            }
        }
        tail.borrow_mut().next = Some(new_head.clone());
        self.head = Some(new_head);
        self.length -= 1;
        Some(value)
    }

    pub fn clear(&mut self) {
        if let Some(head) = self.head.take() {
            let mut current = head.clone();
            loop {
                let next = current.borrow().next.clone();
                current.borrow_mut().next = None;
                if let Some(next_node) = next {
                    if Rc::ptr_eq(&next_node, &head) {
                        break;
                    }
                    current = next_node;
                } else {
                    break;
                }
            }
        }
        self.length = 0;
    }

    pub fn to_vec(&self) -> Vec<T> {
        let mut result = Vec::new();
        if let Some(head) = self.head.clone() {
            let mut current = head.clone();
            loop {
                result.push(current.borrow().value.clone());
                let next = current.borrow().next.clone();
                if let Some(next_node) = next {
                    if Rc::ptr_eq(&next_node, &head) {
                        break;
                    }
                    current = next_node;
                } else {
                    break;
                }
            }
        }
        result
    }
}

pub struct CircularLinkedListIter<T: Clone> {
    current: Option<Rc<RefCell<CircularListNode<T>>>>,
    head: Option<Rc<RefCell<CircularListNode<T>>>>,
    count: usize,
    length: usize,
}

impl<T: Clone> Iterator for CircularLinkedListIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= self.length {
            return None;
        }
        self.current.take().map(|node| {
            let value = node.borrow().value.clone();
            self.current = node.borrow().next.clone();
            self.count += 1;
            value
        })
    }
}

impl<T: Clone> IntoIterator for CircularLinkedList<T> {
    type Item = T;
    type IntoIter = CircularLinkedListIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        CircularLinkedListIter {
            current: self.head.clone(),
            head: self.head.clone(),
            count: 0,
            length: self.length,
        }
    }
}

impl<T: Clone + fmt::Display> fmt::Display for CircularLinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let elements: Vec<String> = self.to_vec().iter().map(|x| x.to_string()).collect();
        write!(f, "[{}]", elements.join(", "))
    }
}

// 双向链表节点
struct DoublyListNode<T: Clone> {
    value: T,
    next: Option<Box<DoublyListNode<T>>>,
    prev: Option<*mut DoublyListNode<T>>,
}

pub struct DoublyLinkedList<T: Clone> {
    head: Option<Box<DoublyListNode<T>>>,
    tail: Option<*mut DoublyListNode<T>>,
    length: usize,
}

impl<T: Clone> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    pub fn tail(&self) -> Option<&T> {
        if let Some(tail) = self.tail {
            unsafe {
                Some(&(*tail).value)
            }
        } else {
            None
        }
    }

    pub fn prepend(&mut self, value: T) {
        let new_node = Box::new(DoublyListNode {
            value,
            next: self.head.take(),
            prev: None,
        });

        let raw_node = Box::into_raw(new_node);

        if let Some(head) = unsafe { &mut (*raw_node).next } {
            head.prev = Some(raw_node);
        } else {
            self.tail = Some(raw_node);
        }

        self.head = Some(unsafe { Box::from_raw(raw_node) });
        self.length += 1;
    }

    pub fn append(&mut self, value: T) {
        let new_node = Box::new(DoublyListNode {
            value,
            next: None,
            prev: self.tail,
        });

        let raw_node = Box::into_raw(new_node);

        if let Some(tail) = self.tail {
            unsafe {
                (*tail).next = Some(Box::from_raw(raw_node));
            }
        } else {
            self.head = Some(unsafe { Box::from_raw(raw_node) });
        }

        self.tail = Some(raw_node);
        self.length += 1;
    }

    pub fn remove_first(&mut self) -> Option<T> {
        self.head.take().map(|mut node| {
            let value = node.value.clone();
            self.head = node.next.take();
            if let Some(ref mut head) = self.head {
                head.prev = None;
            } else {
                self.tail = None;
            }
            self.length -= 1;
            value
        })
    }

    pub fn remove_last(&mut self) -> Option<T> {
        if self.tail.is_none() {
            return None;
        }

        let value = unsafe {
            let tail = self.tail.unwrap();
            let value = (*tail).value.clone();

            if let Some(prev) = (*tail).prev {
                (*prev).next = None;
                self.tail = Some(prev);
            } else {
                self.head = None;
                self.tail = None;
            }

            value
        };

        self.length -= 1;
        Some(value)
    }

    pub fn clear(&mut self) {
        self.head = None;
        self.tail = None;
        self.length = 0;
    }

    pub fn to_vec(&self) -> Vec<T> {
        let mut result = Vec::new();
        let mut current = &self.head;
        while let Some(node) = current {
            result.push(node.value.clone());
            current = &node.next;
        }
        result
    }

    pub fn to_vec_reverse(&self) -> Vec<T> {
        let mut result = Vec::new();
        if self.tail.is_none() {
            return result;
        }

        unsafe {
            let mut current = self.tail;
            for _ in 0..self.length {
                if let Some(node) = current {
                    result.push((*node).value.clone());
                    current = (*node).prev;
                }
            }
        }

        result
    }
}

pub struct DoublyLinkedListIter<T: Clone> {
    current: Option<*const DoublyListNode<T>>,
}

impl<T: Clone> Iterator for DoublyLinkedListIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node_ptr| unsafe {
            let node = &*node_ptr;
            self.current = node.next.as_ref().map(|next| &**next as *const _);
            node.value.clone()
        })
    }
}

impl<T: Clone> IntoIterator for DoublyLinkedList<T> {
    type Item = T;
    type IntoIter = DoublyLinkedListIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        DoublyLinkedListIter {
            current: self.head.as_ref().map(|node| &**node as *const _),
        }
    }
}

impl<'a, T: Clone> IntoIterator for &'a DoublyLinkedList<T> {
    type Item = &'a T;
    type IntoIter = DoublyLinkedListRefIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        DoublyLinkedListRefIter {
            current: self.head.as_deref(),
        }
    }
}

pub struct DoublyLinkedListRefIter<'a, T: Clone> {
    current: Option<&'a DoublyListNode<T>>,
}

impl<'a, T: Clone> Iterator for DoublyLinkedListRefIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node| {
            self.current = node.next.as_deref();
            &node.value
        })
    }
}

impl<T: Clone + fmt::Display> fmt::Display for DoublyLinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let elements: Vec<String> = self.to_vec().iter().map(|x| x.to_string()).collect();
        write!(f, "[{}]", elements.join(", "))
    }
}

// 双向循环链表节点
struct DoublyCircularListNode<T: Clone> {
    value: T,
    next: Option<Rc<RefCell<DoublyCircularListNode<T>>>>,
    prev: Weak<RefCell<DoublyCircularListNode<T>>>,
}

pub struct DoublyCircularLinkedList<T: Clone> {
    head: Option<Rc<RefCell<DoublyCircularListNode<T>>>>,
    length: usize,
}

impl<T: Clone> DoublyCircularLinkedList<T> {
    pub fn new() -> Self {
        DoublyCircularLinkedList {
            head: None,
            length: 0,
        }
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn head(&self) -> Option<T> {
        self.head.as_ref().map(|node| node.borrow().value.clone())
    }

    pub fn tail(&self) -> Option<T> {
        self.head.as_ref().map(|head| {
            head.borrow().prev.upgrade().map(|tail| tail.borrow().value.clone())
        }).flatten()
    }

    pub fn prepend(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(DoublyCircularListNode {
            value,
            next: None,
            prev: Weak::new(),
        }));

        if let Some(head) = self.head.take() {
            let tail = head.borrow().prev.upgrade().unwrap();
            
            new_node.borrow_mut().next = Some(head.clone());
            new_node.borrow_mut().prev = Rc::downgrade(&tail);
            
            head.borrow_mut().prev = Rc::downgrade(&new_node);
            tail.borrow_mut().next = Some(new_node.clone());
            
            self.head = Some(new_node);
        } else {
            new_node.borrow_mut().next = Some(new_node.clone());
            new_node.borrow_mut().prev = Rc::downgrade(&new_node);
            self.head = Some(new_node);
        }
        self.length += 1;
    }

    pub fn append(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(DoublyCircularListNode {
            value,
            next: None,
            prev: Weak::new(),
        }));

        if let Some(head) = self.head.clone() {
            let tail = head.borrow().prev.upgrade().unwrap();
            
            new_node.borrow_mut().next = Some(head.clone());
            new_node.borrow_mut().prev = Rc::downgrade(&tail);
            
            tail.borrow_mut().next = Some(new_node.clone());
            head.borrow_mut().prev = Rc::downgrade(&new_node);
        } else {
            new_node.borrow_mut().next = Some(new_node.clone());
            new_node.borrow_mut().prev = Rc::downgrade(&new_node);
            self.head = Some(new_node);
        }
        self.length += 1;
    }

    pub fn remove_first(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }

        let old_head = self.head.take().unwrap();
        let value = old_head.borrow().value.clone();

        if self.length == 1 {
            self.length = 0;
            return Some(value);
        }

        let new_head = old_head.borrow().next.clone().unwrap();
        let tail = old_head.borrow().prev.upgrade().unwrap();
        
        new_head.borrow_mut().prev = Rc::downgrade(&tail);
        tail.borrow_mut().next = Some(new_head.clone());
        
        old_head.borrow_mut().next = None;
        old_head.borrow_mut().prev = Weak::new();
        
        self.head = Some(new_head);
        self.length -= 1;
        Some(value)
    }

    pub fn remove_last(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }

        let head = self.head.clone().unwrap();
        let old_tail = head.borrow().prev.upgrade().unwrap();
        let value = old_tail.borrow().value.clone();

        if self.length == 1 {
            self.head = None;
            self.length = 0;
            return Some(value);
        }

        let new_tail = old_tail.borrow().prev.upgrade().unwrap();
        
        new_tail.borrow_mut().next = Some(head.clone());
        head.borrow_mut().prev = Rc::downgrade(&new_tail);
        
        old_tail.borrow_mut().next = None;
        old_tail.borrow_mut().prev = Weak::new();
        
        self.length -= 1;
        Some(value)
    }

    pub fn clear(&mut self) {
        if let Some(head) = self.head.take() {
            let mut current = head.clone();
            loop {
                let next = current.borrow().next.clone();
                current.borrow_mut().next = None;
                current.borrow_mut().prev = Weak::new();
                if let Some(next_node) = next {
                    if Rc::ptr_eq(&next_node, &head) {
                        break;
                    }
                    current = next_node;
                } else {
                    break;
                }
            }
        }
        self.length = 0;
    }

    pub fn to_vec(&self) -> Vec<T> {
        let mut result = Vec::new();
        if let Some(head) = self.head.clone() {
            let mut current = head.clone();
            loop {
                result.push(current.borrow().value.clone());
                let next = current.borrow().next.clone();
                if let Some(next_node) = next {
                    if Rc::ptr_eq(&next_node, &head) {
                        break;
                    }
                    current = next_node;
                } else {
                    break;
                }
            }
        }
        result
    }

    pub fn to_vec_reverse(&self) -> Vec<T> {
        let mut result = Vec::new();
        if let Some(head) = self.head.clone() {
            let tail = head.borrow().prev.upgrade().unwrap();
            let mut current = tail.clone();
            loop {
                result.push(current.borrow().value.clone());
                let prev = current.borrow().prev.upgrade();
                if let Some(prev_node) = prev {
                    if Rc::ptr_eq(&prev_node, &tail) {
                        break;
                    }
                    current = prev_node;
                } else {
                    break;
                }
            }
        }
        result
    }
}

pub struct DoublyCircularLinkedListIter<T: Clone> {
    current: Option<Rc<RefCell<DoublyCircularListNode<T>>>>,
    count: usize,
    length: usize,
}

impl<T: Clone> Iterator for DoublyCircularLinkedListIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= self.length {
            return None;
        }
        self.current.take().map(|node| {
            let value = node.borrow().value.clone();
            self.current = node.borrow().next.clone();
            self.count += 1;
            value
        })
    }
}

impl<T: Clone> IntoIterator for DoublyCircularLinkedList<T> {
    type Item = T;
    type IntoIter = DoublyCircularLinkedListIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        DoublyCircularLinkedListIter {
            current: self.head.clone(),
            count: 0,
            length: self.length,
        }
    }
}

impl<T: Clone + fmt::Display> fmt::Display for DoublyCircularLinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let elements: Vec<String> = self.to_vec().iter().map(|x| x.to_string()).collect();
        write!(f, "[{}]", elements.join(", "))
    }
}

fn main() {
    println!("Singly Linked List Example");
    let mut sll = SinglyLinkedList::<i32>::new();
    sll.append(1);
    sll.append(2);
    sll.append(3);
    println!("List: {}", sll);
    println!("Iterating with for loop:");
    for val in &sll {
        print!("{} ", val);
    }
    println!();

    println!("\nCircular Linked List Example");
    let mut cll = CircularLinkedList::<i32>::new();
    cll.append(1);
    cll.append(2);
    cll.append(3);
    println!("List: {}", cll);
    println!("Iterating with for loop:");
    for val in cll {
        print!("{} ", val);
    }
    println!();

    println!("\nDoubly Linked List Example");
    let mut dll = DoublyLinkedList::<i32>::new();
    dll.append(1);
    dll.append(2);
    dll.append(3);
    println!("List: {}", dll);
    println!("Reverse: {:?}", dll.to_vec_reverse());
    println!("Iterating with for loop:");
    for val in &dll {
        print!("{} ", val);
    }
    println!();

    println!("\nDoubly Circular Linked List Example");
    let mut dcll = DoublyCircularLinkedList::<i32>::new();
    dcll.append(1);
    dcll.append(2);
    dcll.append(3);
    println!("List: {}", dcll);
    println!("Reverse: {:?}", dcll.to_vec_reverse());
    println!("Iterating with for loop:");
    for val in dcll {
        print!("{} ", val);
    }
    println!();
}
