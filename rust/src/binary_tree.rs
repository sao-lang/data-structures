#![allow(dead_code)]

use std::fmt;
use std::cmp::Ordering;

struct TreeNode<T: Clone + PartialOrd> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

pub struct BinaryTree<T: Clone + PartialOrd> {
    root: Option<Box<TreeNode<T>>>,
}

impl<T: Clone + PartialOrd + fmt::Display> BinaryTree<T> {
    pub fn new() -> Self {
        BinaryTree { root: None }
    }

    pub fn root(&self) -> Option<&T> {
        self.root.as_ref().map(|node| &node.value)
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn pre_order_traversal(&self) -> Vec<T> {
        let mut result = Vec::new();
        self.pre_order_helper(&self.root, &mut result);
        result
    }

    fn pre_order_helper(&self, node: &Option<Box<TreeNode<T>>>, result: &mut Vec<T>) {
        if let Some(n) = node {
            result.push(n.value.clone());
            self.pre_order_helper(&n.left, result);
            self.pre_order_helper(&n.right, result);
        }
    }

    pub fn in_order_traversal(&self) -> Vec<T> {
        let mut result = Vec::new();
        self.in_order_helper(&self.root, &mut result);
        result
    }

    fn in_order_helper(&self, node: &Option<Box<TreeNode<T>>>, result: &mut Vec<T>) {
        if let Some(n) = node {
            self.in_order_helper(&n.left, result);
            result.push(n.value.clone());
            self.in_order_helper(&n.right, result);
        }
    }

    pub fn post_order_traversal(&self) -> Vec<T> {
        let mut result = Vec::new();
        self.post_order_helper(&self.root, &mut result);
        result
    }

    fn post_order_helper(&self, node: &Option<Box<TreeNode<T>>>, result: &mut Vec<T>) {
        if let Some(n) = node {
            self.post_order_helper(&n.left, result);
            self.post_order_helper(&n.right, result);
            result.push(n.value.clone());
        }
    }

    pub fn level_order_traversal(&self) -> Vec<T> {
        let mut result = Vec::new();
        if self.root.is_none() {
            return result;
        }

        let mut queue = vec![self.root.as_ref().unwrap()];
        let mut i = 0;

        while i < queue.len() {
            let node = queue[i];
            result.push(node.value.clone());
            if let Some(left) = &node.left {
                queue.push(left);
            }
            if let Some(right) = &node.right {
                queue.push(right);
            }
            i += 1;
        }

        result
    }

    pub fn height(&self) -> i32 {
        self.height_helper(&self.root)
    }

    fn height_helper(&self, node: &Option<Box<TreeNode<T>>>) -> i32 {
        node.as_ref().map_or(-1, |n| {
            let left_height = self.height_helper(&n.left);
            let right_height = self.height_helper(&n.right);
            (if left_height > right_height { left_height } else { right_height }) + 1
        })
    }

    pub fn size(&self) -> usize {
        self.size_helper(&self.root)
    }

    fn size_helper(&self, node: &Option<Box<TreeNode<T>>>) -> usize {
        node.as_ref().map_or(0, |n| {
            1 + self.size_helper(&n.left) + self.size_helper(&n.right)
        })
    }

    pub fn clear(&mut self) {
        self.root = None;
    }
}

pub struct BinarySearchTree<T: Clone + PartialOrd + fmt::Display> {
    tree: BinaryTree<T>,
}

impl<T: Clone + PartialOrd + fmt::Display> BinarySearchTree<T> {
    pub fn new() -> Self {
        BinarySearchTree {
            tree: BinaryTree::new(),
        }
    }

    pub fn insert(&mut self, value: T) {
        let new_node = Box::new(TreeNode {
            value,
            left: None,
            right: None,
        });

        if self.tree.root.is_none() {
            self.tree.root = Some(new_node);
            return;
        }

        let mut current = self.tree.root.as_mut().unwrap();
        loop {
            match new_node.value.partial_cmp(&current.value) {
                Some(Ordering::Less) => {
                    if current.left.is_none() {
                        current.left = Some(new_node);
                        break;
                    } else {
                        current = current.left.as_mut().unwrap();
                    }
                }
                Some(Ordering::Greater) => {
                    if current.right.is_none() {
                        current.right = Some(new_node);
                        break;
                    } else {
                        current = current.right.as_mut().unwrap();
                    }
                }
                _ => break,
            }
        }
    }

    pub fn search(&self, value: &T) -> bool {
        let mut current = &self.tree.root;
        while let Some(node) = current {
            match value.partial_cmp(&node.value) {
                Some(Ordering::Less) => current = &node.left,
                Some(Ordering::Greater) => current = &node.right,
                _ => return true,
            }
        }
        false
    }

    pub fn find_min(&self) -> Option<&T> {
        let mut current = &self.tree.root;
        while let Some(node) = current {
            if node.left.is_none() {
                return Some(&node.value);
            }
            current = &node.left;
        }
        None
    }

    pub fn find_max(&self) -> Option<&T> {
        let mut current = &self.tree.root;
        while let Some(node) = current {
            if node.right.is_none() {
                return Some(&node.value);
            }
            current = &node.right;
        }
        None
    }
}

fn main() {
    println!("Binary Search Tree Example:");
    let mut bst = BinarySearchTree::<i32>::new();
    bst.insert(5);
    bst.insert(3);
    bst.insert(7);
    bst.insert(2);
    bst.insert(4);
    bst.insert(6);
    bst.insert(8);

    println!("In-order traversal: {:?}", bst.tree.in_order_traversal());
    println!("Pre-order traversal: {:?}", bst.tree.pre_order_traversal());
    println!("Post-order traversal: {:?}", bst.tree.post_order_traversal());
    println!("Level-order traversal: {:?}", bst.tree.level_order_traversal());
    if let Some(min) = bst.find_min() {
        println!("Min: {}", min);
    }
    if let Some(max) = bst.find_max() {
        println!("Max: {}", max);
    }
    println!("Search for 4: {}", bst.search(&4));
    println!("Search for 9: {}", bst.search(&9));
    println!("Height: {}", bst.tree.height());
    println!("Size: {}", bst.tree.size());
}
