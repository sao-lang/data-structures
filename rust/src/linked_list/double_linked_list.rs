#![allow(dead_code)]

use std::{
    cell::RefCell,
    fmt::Debug,
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
        let mut current = self.head.clone();
        let mut index = 0;
        loop {
            let node_rc = match current.take() {
                None => {
                    return None;
                }
                Some(node) => node,
            };
            if node_rc.borrow().data == data {
                return Some(index);
            }
            current = node_rc.borrow_mut().next.clone();
            index += 1;
        }
    }

    pub fn display_forward(&self)
    where
        T: Debug,
    {
        let mut current = self.head.clone();
        let mut result = String::new();
        while let Some(node) = current {
            result.push_str(format!("{:?}", node.borrow().data).as_str());
            if node.borrow().next.is_some() {
                result.push_str(format!("{:?}", "<=>").as_str());
            }
            current = node.borrow().next.clone();
        }
        print!("{:?}", result);
    }

    pub fn display_back(&self)
    where
        T: Debug,
    {
        let mut current = self.tail.clone();
        let mut result = String::new();
        while let Some(node) = current {
            result.push_str(format!("{:?}", node.borrow().data).as_str());
            if node.borrow().prev.is_some() {
                result.push_str(format!("{:?}", "<=>").as_str());
            }
            current = match node.borrow().prev.clone() {
                None => None,
                // 这里是将弱引用升级，返回一个新的强引用
                Some(weak) => weak.upgrade(),
            };
        }
        print!("{:?}", result);
    }
}

// 迭代器的实现，Iterator是让当前结构可以遍历自身的元素，IntoIterator是让当前的结构变成一个集合，用于for循环遍历

// impl<'a, T> Iterator for DoubleLinked<'a, T>
// where
//     // 约束：要求数据 T 的生命周期至少和迭代器一样长
//     T: 'a,
// {
//     // 关联类型：定义每次迭代返回的元素类型
//     type Item = &'a T;

//     /// 核心方法：返回下一个元素，并将指针向前移动一位
//     fn next(&mut self) -> Option<Self::Item> {
//         // 1. 使用 self.next.take() 消耗 Option，获取当前节点 Rc 的引用
//         self.next.take().map(|node_rc| {

//             // node_rc 是 &'a Rc<RefCell<DoubleLinkedNode<T>>>

//             // 2. 更新 self.next 到下一个节点的引用
//             //    node_rc.borrow() 创建一个临时的 Ref 借用，我们从中安全地获取 next 字段的引用。
//             //    这个引用 &'a Rc<...> 的生命周期是绑定到 node_rc (即链表节点) 上的，而不是临时的 Ref。
//             self.next = node_rc.borrow().next.as_ref();

//             // 3. 返回当前节点数据的引用
//             //    &node_rc.borrow().data 返回 &'a T，这是安全的，因为它借用了链表节点内部的数据。
//             &node_rc.borrow().data
//         })
//     }
// }

// // 为不可变引用 (&'a DoubleLinkedList<T>) 实现 IntoIterator
// impl<'a, T> IntoIterator for &'a DoubleLinkedList<T> {
//     // 关联类型：定义这个迭代器返回的实际类型
//     type Item = &'a T;

//     // 关联类型：定义返回的迭代器类型
//     type IntoIter = DoubleLinked<'a, T>;

//     /// 核心方法：将集合转换为迭代器
//     fn into_iter(self) -> Self::IntoIter {
//         DoubleLinked {
//             // 返回迭代器，初始指向链表的 head 节点
//             next: self.head.as_ref(), // 使用 as_ref() 避免 clone()
//         }
//     }
// }

// // 假设您已经创建并填充了链表
// // let list: DoubleLinkedList<i32> = ...;

// // 1. 使用 for 循环 (最推荐)
// // for x 会是 &i32 类型
// for x in &list {
//     println!("元素: {}", x);
// }

// // 2. 使用迭代器方法 (例如：求和)
// let sum: i32 = list.iter().sum(); // 自动获得了 sum() 方法

// // 3. 像您之前的问题一样，实现 display_forward
// pub fn display_forward_final(&self)
// where
//     T: Debug,
// {
//     let result = self
//         .iter() // 调用我们实现的迭代器
//         .map(|data_ref| format!("{:?}", data_ref))
//         .collect::<Vec<String>>()
//         .join(" <=> ");

//     println!("{}", result);
// }

// 其他还有Debug Copy Clone Eq PartialEq，都可以进行实现
