#![allow(dead_code)]
// #![allow(unused_assignments)]

use std::fmt;
use std::cmp::Ordering;
use std::collections::{VecDeque, HashMap, HashSet};
use std::rc::Rc;
use std::cell::RefCell;
use std::rc::Weak;
// use rand::Rng;

#[derive(Debug, PartialEq, Eq)]
pub enum AdvancedError {
    DequeIsEmpty,
}

impl fmt::Display for AdvancedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AdvancedError::DequeIsEmpty => write!(f, "Deque is empty"),
        }
    }
}

impl std::error::Error for AdvancedError {}

#[derive(Clone)]
struct AVLTreeNode<T: Clone + PartialOrd> {
    value: T,
    left: Option<Box<AVLTreeNode<T>>>,
    right: Option<Box<AVLTreeNode<T>>>,
    height: i32,
}

pub struct AVLTree<T: Clone + PartialOrd> {
    root: Option<Box<AVLTreeNode<T>>>,
}

impl<T: Clone + PartialOrd + fmt::Display> AVLTree<T> {
    pub fn new() -> Self {
        AVLTree { root: None }
    }

    pub fn root(&self) -> Option<&T> {
        self.root.as_ref().map(|node| &node.value)
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    fn get_height(node: &Option<Box<AVLTreeNode<T>>>) -> i32 {
        node.as_ref().map_or(0, |n| n.height)
    }

    fn get_balance(node: &Option<Box<AVLTreeNode<T>>>) -> i32 {
        node.as_ref().map_or(0, |n| Self::get_height(&n.left) - Self::get_height(&n.right))
    }

    fn right_rotate(mut y: Box<AVLTreeNode<T>>) -> Box<AVLTreeNode<T>> {
        let mut x = y.left.take().unwrap();
        let t2 = x.right.take();

        x.right = Some(y);
        x.right.as_mut().unwrap().left = t2;

        let y_left = Self::get_height(&x.right.as_ref().unwrap().left);
        let y_right = Self::get_height(&x.right.as_ref().unwrap().right);
        x.right.as_mut().unwrap().height = (if y_left > y_right { y_left } else { y_right }) + 1;
        let x_left = Self::get_height(&x.left);
        let x_right = Self::get_height(&x.right);
        x.height = (if x_left > x_right { x_left } else { x_right }) + 1;

        x
    }

    fn left_rotate(mut x: Box<AVLTreeNode<T>>) -> Box<AVLTreeNode<T>> {
        let mut y = x.right.take().unwrap();
        let t2 = y.left.take();

        y.left = Some(x);
        y.left.as_mut().unwrap().right = t2;

        let x_left = Self::get_height(&y.left.as_ref().unwrap().left);
        let x_right = Self::get_height(&y.left.as_ref().unwrap().right);
        y.left.as_mut().unwrap().height = (if x_left > x_right { x_left } else { x_right }) + 1;
        let y_left = Self::get_height(&y.left);
        let y_right = Self::get_height(&y.right);
        y.height = (if y_left > y_right { y_left } else { y_right }) + 1;

        y
    }

    fn insert_helper(node: Option<Box<AVLTreeNode<T>>>, value: T) -> Box<AVLTreeNode<T>> {
        let value_clone = value.clone();
        let mut node = match node {
            Some(n) => n,
            None => {
                return Box::new(AVLTreeNode {
                    value,
                    left: None,
                    right: None,
                    height: 1,
                });
            }
        };

        match value_clone.partial_cmp(&node.value) {
            Some(Ordering::Less) => {
                node.left = Some(Self::insert_helper(node.left.take(), value_clone.clone()));
            }
            Some(Ordering::Greater) => {
                node.right = Some(Self::insert_helper(node.right.take(), value_clone.clone()));
            }
            _ => return node,
        }

        let left = Self::get_height(&node.left);
        let right = Self::get_height(&node.right);
        node.height = (if left > right { left } else { right }) + 1;

        let balance = Self::get_balance(&Some(node.clone()));

        if balance > 1 {
            if let Some(left) = &node.left {
                if value_clone < left.value {
                    return Self::right_rotate(node);
                } else {
                    node.left = Some(Self::left_rotate(node.left.take().unwrap()));
                    return Self::right_rotate(node);
                }
            }
        }

        if balance < -1 {
            if let Some(right) = &node.right {
                if value_clone > right.value {
                    return Self::left_rotate(node);
                } else {
                    node.right = Some(Self::right_rotate(node.right.take().unwrap()));
                    return Self::left_rotate(node);
                }
            }
        }

        node
    }

    pub fn insert(&mut self, value: T) {
        self.root = Some(Self::insert_helper(self.root.take(), value));
    }

    pub fn search(&self, value: &T) -> bool {
        let mut current = &self.root;
        while let Some(node) = current {
            match value.partial_cmp(&node.value) {
                Some(Ordering::Less) => current = &node.left,
                Some(Ordering::Greater) => current = &node.right,
                _ => return true,
            }
        }
        false
    }

    pub fn pre_order_traversal(&self) -> Vec<T> {
        let mut result = Vec::new();
        self.pre_order_helper(&self.root, &mut result);
        result
    }

    fn pre_order_helper(&self, node: &Option<Box<AVLTreeNode<T>>>, result: &mut Vec<T>) {
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

    fn in_order_helper(&self, node: &Option<Box<AVLTreeNode<T>>>, result: &mut Vec<T>) {
        if let Some(n) = node {
            self.in_order_helper(&n.left, result);
            result.push(n.value.clone());
            self.in_order_helper(&n.right, result);
        }
    }

    pub fn level_order_traversal(&self) -> Vec<T> {
        let mut result = Vec::new();
        if self.root.is_none() {
            return result;
        }

        let mut queue = VecDeque::new();
        queue.push_back(self.root.as_ref().unwrap());

        while let Some(node) = queue.pop_front() {
            result.push(node.value.clone());
            if let Some(left) = &node.left {
                queue.push_back(left);
            }
            if let Some(right) = &node.right {
                queue.push_back(right);
            }
        }

        result
    }

    pub fn height(&self) -> i32 {
        Self::get_height(&self.root)
    }

    pub fn clear(&mut self) {
        self.root = None;
    }
}

struct TrieNode {
    children: HashMap<char, Box<TrieNode>>,
    is_end_of_word: bool,
}

pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode {
                children: HashMap::new(),
                is_end_of_word: false,
            },
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut current = &mut self.root;
        for c in word.chars() {
            current = current.children.entry(c).or_insert_with(|| {
                Box::new(TrieNode {
                    children: HashMap::new(),
                    is_end_of_word: false,
                })
            });
        }
        current.is_end_of_word = true;
    }

    pub fn search(&self, word: &str) -> bool {
        let mut current = &self.root;
        for c in word.chars() {
            match current.children.get(&c) {
                Some(node) => current = node,
                None => return false,
            }
        }
        current.is_end_of_word
    }

    pub fn starts_with(&self, prefix: &str) -> bool {
        let mut current = &self.root;
        for c in prefix.chars() {
            match current.children.get(&c) {
                Some(node) => current = node,
                None => return false,
            }
        }
        true
    }

    pub fn get_all_words(&self) -> Vec<String> {
        let mut result = Vec::new();
        self.get_all_words_helper(&self.root, String::new(), &mut result);
        result
    }

    fn get_all_words_helper(&self, node: &TrieNode, prefix: String, result: &mut Vec<String>) {
        if node.is_end_of_word {
            result.push(prefix.clone());
        }
        for (c, child) in &node.children {
            let mut new_prefix = prefix.clone();
            new_prefix.push(*c);
            self.get_all_words_helper(child, new_prefix, result);
        }
    }
}

struct GraphNode<T: Clone + PartialEq + Eq + std::hash::Hash> {
    value: T,
    neighbors: Vec<T>,
}

pub struct Graph<T: Clone + PartialEq + Eq + std::hash::Hash> {
    nodes: HashMap<T, GraphNode<T>>,
    is_directed: bool,
}

impl<T: Clone + PartialEq + Eq + std::hash::Hash + fmt::Display> Graph<T> {
    pub fn new(is_directed: bool) -> Self {
        Graph {
            nodes: HashMap::new(),
            is_directed,
        }
    }

    pub fn add_vertex(&mut self, value: T) {
        self.nodes.entry(value.clone()).or_insert_with(|| GraphNode {
            value,
            neighbors: Vec::new(),
        });
    }

    pub fn add_edge(&mut self, from: T, to: T) {
        self.add_vertex(from.clone());
        self.add_vertex(to.clone());

        if let Some(node) = self.nodes.get_mut(&from) {
            node.neighbors.push(to.clone());
        }
        if !self.is_directed {
            if let Some(node) = self.nodes.get_mut(&to) {
                node.neighbors.push(from);
            }
        }
    }

    pub fn bfs(&self, start: T) -> Vec<T> {
        let mut result = Vec::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        if !self.nodes.contains_key(&start) {
            return result;
        }

        visited.insert(start.clone());
        queue.push_back(start);

        while let Some(current) = queue.pop_front() {
            result.push(current.clone());
            if let Some(node) = self.nodes.get(&current) {
                for neighbor in &node.neighbors {
                    if !visited.contains(neighbor) {
                        visited.insert(neighbor.clone());
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }

        result
    }

    pub fn dfs(&self, start: T) -> Vec<T> {
        let mut result = Vec::new();
        let mut visited = HashSet::new();
        let mut stack = Vec::new();

        if !self.nodes.contains_key(&start) {
            return result;
        }

        stack.push(start.clone());
        visited.insert(start);

        while let Some(current) = stack.pop() {
            result.push(current.clone());
            if let Some(node) = self.nodes.get(&current) {
                for neighbor in node.neighbors.iter().rev() {
                    if !visited.contains(neighbor) {
                        visited.insert(neighbor.clone());
                        stack.push(neighbor.clone());
                    }
                }
            }
        }

        result
    }

    pub fn get_vertices(&self) -> Vec<T> {
        self.nodes.keys().cloned().collect()
    }

    pub fn has_vertex(&self, value: &T) -> bool {
        self.nodes.contains_key(value)
    }
}

pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        let parent: Vec<usize> = (0..size).collect();
        UnionFind {
            parent,
            rank: vec![0; size],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false;
        }

        if self.rank[root_x] < self.rank[root_y] {
            self.parent[root_x] = root_y;
        } else if self.rank[root_x] > self.rank[root_y] {
            self.parent[root_y] = root_x;
        } else {
            self.parent[root_y] = root_x;
            self.rank[root_x] += 1;
        }

        true
    }

    pub fn connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    pub fn get_count(&mut self) -> usize {
        let mut roots = HashSet::new();
        for i in 0..self.parent.len() {
            roots.insert(self.find(i));
        }
        roots.len()
    }
}



pub struct Deque<T: Clone> {
    items: Vec<T>,
}

impl<T: Clone> Deque<T> {
    pub fn new() -> Self {
        Deque { items: Vec::new() }
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn add_first(&mut self, item: T) {
        self.items.insert(0, item);
    }

    pub fn add_last(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn remove_first(&mut self) -> Result<T, AdvancedError> {
        if self.is_empty() {
            return Err(AdvancedError::DequeIsEmpty);
        }
        Ok(self.items.remove(0))
    }

    pub fn remove_last(&mut self) -> Result<T, AdvancedError> {
        self.items.pop().ok_or(AdvancedError::DequeIsEmpty)
    }

    pub fn peek_first(&self) -> Result<&T, AdvancedError> {
        self.items.first().ok_or(AdvancedError::DequeIsEmpty)
    }

    pub fn peek_last(&self) -> Result<&T, AdvancedError> {
        self.items.last().ok_or(AdvancedError::DequeIsEmpty)
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn to_vec(&self) -> Vec<T> {
        self.items.clone()
    }
}

impl<T: Clone + fmt::Display> fmt::Display for Deque<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let elements: Vec<String> = self.items.iter().map(|x| x.to_string()).collect();
        write!(f, "[{}]", elements.join(", "))
    }
}

pub struct FenwickTree {
    tree: Vec<i64>,
    n: usize,
}

impl FenwickTree {
    pub fn new(size: usize) -> Self {
        FenwickTree {
            tree: vec![0; size + 1],
            n: size,
        }
    }

    pub fn from_data(data: &[i64]) -> Self {
        let mut ft = FenwickTree::new(data.len());
        for i in 0..data.len() {
            ft.update(i, data[i]);
        }
        ft
    }

    pub fn update(&mut self, index: usize, delta: i64) {
        let mut i = (index + 1) as i32;
        while i <= self.n as i32 {
            self.tree[i as usize] += delta;
            i += i & -i;
        }
    }

    pub fn set(&mut self, index: usize, value: i64) {
        let current = self.query(index, index);
        self.update(index, value - current);
    }

    pub fn prefix_sum(&self, index: usize) -> i64 {
        let mut sum = 0;
        let mut i = (index + 1) as i32;
        while i > 0 {
            sum += self.tree[i as usize];
            i -= i & -i;
        }
        sum
    }

    pub fn query(&self, l: usize, r: usize) -> i64 {
        if l == 0 {
            self.prefix_sum(r)
        } else {
            self.prefix_sum(r) - self.prefix_sum(l - 1)
        }
    }

    pub fn size(&self) -> usize {
        self.n
    }
}

pub struct SegmentTree<T> {
    n: usize,
    size: usize,
    tree: Vec<T>,
    merge: fn(T, T) -> T,
    default_value: T,
}

impl<T: Clone + Copy + PartialEq> SegmentTree<T> {
    pub fn new(data: &[T], merge: fn(T, T) -> T, default_value: T) -> Self {
        let n = data.len();
        let mut size = 1;
        while size < n {
            size <<= 1;
        }

        let mut tree = vec![default_value; 2 * size];
        for i in 0..n {
            tree[size + i] = data[i];
        }

        for i in (1..size).rev() {
            tree[i] = merge(tree[2 * i], tree[2 * i + 1]);
        }

        SegmentTree {
            n,
            size,
            tree,
            merge,
            default_value,
        }
    }

    pub fn update(&mut self, index: usize, value: T) {
        let mut idx = index + self.size;
        self.tree[idx] = value;
        idx >>= 1;

        while idx >= 1 {
            let new_val = (self.merge)(self.tree[2 * idx], self.tree[2 * idx + 1]);
            if self.tree[idx] == new_val {
                break;
            }
            self.tree[idx] = new_val;
            idx >>= 1;
        }
    }

    pub fn query(&self, l: usize, r: usize) -> T {
        let mut res_left = self.default_value;
        let mut res_right = self.default_value;
        let mut l = l + self.size;
        let mut r = r + self.size;

        while l <= r {
            if l % 2 == 1 {
                res_left = (self.merge)(res_left, self.tree[l]);
                l += 1;
            }
            if r % 2 == 0 {
                res_right = (self.merge)(self.tree[r], res_right);
                r -= 1;
            }
            l >>= 1;
            r >>= 1;
        }

        (self.merge)(res_left, res_right)
    }

    pub fn get(&self, index: usize) -> T {
        self.tree[self.size + index]
    }
}

pub struct BloomFilter {
    bit_array: Vec<u8>,
    size: usize,
    num_hash_functions: usize,
}

impl BloomFilter {
    pub fn new(expected_items: usize, false_positive_rate: f64) -> Self {
        let size = Self::calculate_size(expected_items, false_positive_rate);
        let num_hash_functions = Self::calculate_num_hash_functions(size, expected_items);
        BloomFilter {
            bit_array: vec![0; (size + 7) / 8],
            size,
            num_hash_functions,
        }
    }

    fn calculate_size(n: usize, p: f64) -> usize {
        let ln2 = (2.0f64).ln();
        let ln2_sq = ln2 * ln2;
        let ln_p = p.ln();
        (- (n as f64) * ln_p / ln2_sq).ceil() as usize
    }

    fn calculate_num_hash_functions(m: usize, n: usize) -> usize {
        let ln2 = (2.0f64).ln();
        let k = ((m as f64 / n as f64) * ln2).round() as usize;
        if k > 1 { k } else { 1 }
    }

    fn hash(&self, item: &str, seed: usize) -> usize {
        let mut hash = seed;
        for c in item.chars() {
            hash = (hash * 31 + c as usize) % self.size;
        }
        hash
    }

    pub fn add(&mut self, item: &str) {
        for i in 0..self.num_hash_functions {
            let hash = self.hash(item, i);
            let byte_index = hash / 8;
            let bit_index = hash % 8;
            self.bit_array[byte_index] |= 1 << bit_index;
        }
    }

    pub fn might_contain(&self, item: &str) -> bool {
        for i in 0..self.num_hash_functions {
            let hash = self.hash(item, i);
            let byte_index = hash / 8;
            let bit_index = hash % 8;
            if (self.bit_array[byte_index] & (1 << bit_index)) == 0 {
                return false;
            }
        }
        true
    }

    pub fn clear(&mut self) {
        self.bit_array.fill(0);
    }
}

struct LRUCacheNode<K: Clone + Eq + std::hash::Hash, V: Clone> {
    key: K,
    value: V,
    prev: Option<Weak<RefCell<LRUCacheNode<K, V>>>>,
    next: Option<Rc<RefCell<LRUCacheNode<K, V>>>>,
}

pub struct LRUCache<K: Clone + Eq + std::hash::Hash, V: Clone> {
    capacity: usize,
    cache: HashMap<K, Rc<RefCell<LRUCacheNode<K, V>>>>,
    head: Option<Rc<RefCell<LRUCacheNode<K, V>>>>,
    tail: Option<Rc<RefCell<LRUCacheNode<K, V>>>>,
}

impl<K: Clone + Eq + std::hash::Hash, V: Clone> LRUCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        LRUCache {
            capacity,
            cache: HashMap::new(),
            head: None,
            tail: None,
        }
    }

    fn add_to_head(&mut self, node: Rc<RefCell<LRUCacheNode<K, V>>>) {
        node.borrow_mut().prev = None;
        node.borrow_mut().next = self.head.clone();

        if let Some(head) = self.head.clone() {
            head.borrow_mut().prev = Some(Rc::downgrade(&node));
        } else {
            self.tail = Some(node.clone());
        }

        self.head = Some(node);
    }

    fn remove_node(&mut self, node: Rc<RefCell<LRUCacheNode<K, V>>>) {
        let prev = node.borrow().prev.clone().and_then(|w| w.upgrade());
        let next = node.borrow().next.clone();

        if let Some(ref p) = prev {
            p.borrow_mut().next = next.clone();
        } else {
            self.head = next.clone();
        }

        if let Some(next) = next {
            next.borrow_mut().prev = prev.as_ref().map(|p| Rc::downgrade(p));
        } else {
            self.tail = prev;
        }

        node.borrow_mut().prev = None;
        node.borrow_mut().next = None;
    }

    fn move_to_head(&mut self, node: Rc<RefCell<LRUCacheNode<K, V>>>) {
        self.remove_node(node.clone());
        self.add_to_head(node);
    }

    pub fn get(&mut self, key: K) -> Option<V> {
        if let Some(node) = self.cache.get(&key) {
            let value = node.borrow().value.clone();
            self.move_to_head(node.clone());
            Some(value)
        } else {
            None
        }
    }

    pub fn put(&mut self, key: K, value: V) {
        if let Some(node) = self.cache.get(&key) {
            node.borrow_mut().value = value;
            self.move_to_head(node.clone());
        } else {
            let new_node = Rc::new(RefCell::new(LRUCacheNode {
                key: key.clone(),
                value,
                prev: None,
                next: None,
            }));

            self.cache.insert(key, new_node.clone());
            self.add_to_head(new_node);

            if self.cache.len() > self.capacity {
                if let Some(tail) = self.tail.clone() {
                    let tail_key = tail.borrow().key.clone();
                    self.remove_node(tail);
                    self.cache.remove(&tail_key);
                }
            }
        }
    }

    pub fn has(&self, key: K) -> bool {
        self.cache.contains_key(&key)
    }

    pub fn size(&self) -> usize {
        self.cache.len()
    }

    pub fn clear(&mut self) {
        self.cache.clear();
        self.head = None;
        self.tail = None;
    }
}

struct SkipListNode<T: Clone + PartialOrd> {
    value: T,
    forward: Vec<Option<Rc<RefCell<SkipListNode<T>>>>>,
}

pub struct SkipList<T: Clone + PartialOrd> {
    head: Rc<RefCell<SkipListNode<T>>>,
    level: usize,
    max_level: usize,
    p: f64,
}

impl<T: Clone + PartialOrd + Default> SkipList<T> {
    const MAX_LEVEL: usize = 16;
    const P: f64 = 0.5;

    pub fn new() -> Self {
        let head = Rc::new(RefCell::new(SkipListNode {
            value: T::default(),
            forward: vec![None; Self::MAX_LEVEL + 1],
        }));

        SkipList {
            head,
            level: 0,
            max_level: Self::MAX_LEVEL,
            p: Self::P,
        }
    }

    fn random_level(&self) -> usize {
        let mut level = 0;
        let mut seed = 1;
        while level < self.max_level - 1 {
            seed = (seed * 1103515245 + 12345) & 0x7fffffff;
            if (seed as f64 / 0x7fffffff as f64) < self.p {
                level += 1;
            } else {
                break;
            }
        }
        level
    }

    pub fn search(&self, value: T) -> bool {
        let mut current = self.head.clone();

        for i in (0..=self.level).rev() {
            loop {
                let next_opt = current.borrow().forward[i].clone();
                match next_opt {
                    Some(next) if next.borrow().value < value => {
                        current = next;
                    }
                    _ => break,
                }
            }
        }

        if let Some(next) = current.borrow().forward[0].clone() {
            next.borrow().value == value
        } else {
            false
        }
    }

    pub fn insert(&mut self, value: T) {
        let mut update = vec![self.head.clone(); self.max_level + 1];
        let mut current = self.head.clone();

        for i in (0..=self.level).rev() {
            loop {
                let next_opt = current.borrow().forward[i].clone();
                match next_opt {
                    Some(next) if next.borrow().value < value => {
                        current = next;
                    }
                    _ => break,
                }
            }
            update[i] = current.clone();
        }

        current = update[0].borrow().forward[0].clone().unwrap_or_else(|| self.head.clone());

        if current.borrow().value != value || Rc::ptr_eq(&current, &self.head) {
            let new_level = self.random_level();

            if new_level > self.level {
                for i in self.level + 1..=new_level {
                    update[i] = self.head.clone();
                }
                self.level = new_level;
            }

            let new_node = Rc::new(RefCell::new(SkipListNode {
                value,
                forward: vec![None; new_level + 1],
            }));

            for i in 0..=new_level {
                new_node.borrow_mut().forward[i] = update[i].borrow().forward[i].clone();
                update[i].borrow_mut().forward[i] = Some(new_node.clone());
            }
        }
    }

    pub fn delete(&mut self, value: T) -> bool {
        let mut update = vec![self.head.clone(); self.max_level + 1];
        let mut current = self.head.clone();

        for i in (0..=self.level).rev() {
            loop {
                let next_opt = current.borrow().forward[i].clone();
                match next_opt {
                    Some(next) if next.borrow().value < value => {
                        current = next;
                    }
                    _ => break,
                }
            }
            update[i] = current.clone();
        }

        current = update[0].borrow().forward[0].clone().unwrap_or_else(|| self.head.clone());

        if current.borrow().value == value && !Rc::ptr_eq(&current, &self.head) {
            for i in 0..=self.level {
                if update[i].borrow().forward[i].as_ref().map_or(true, |n| !Rc::ptr_eq(n, &current)) {
                    break;
                }
                update[i].borrow_mut().forward[i] = current.borrow().forward[i].clone();
            }

            while self.level > 0 && self.head.borrow().forward[self.level].is_none() {
                self.level -= 1;
            }

            return true;
        }

        false
    }

    pub fn to_vec(&self) -> Vec<T> {
        let mut result = Vec::new();
        let mut current = self.head.borrow().forward[0].clone();
        while let Some(node) = current {
            result.push(node.borrow().value.clone());
            current = node.borrow().forward[0].clone();
        }
        result
    }
}

pub struct SuffixArray {
    text: String,
    suffix_array: Vec<usize>,
    lcp_array: Option<Vec<usize>>,
}

impl SuffixArray {
    pub fn new(text: String) -> Self {
        let suffix_array = Self::build_suffix_array(&text);
        SuffixArray {
            text,
            suffix_array,
            lcp_array: None,
        }
    }

    fn build_suffix_array(s: &str) -> Vec<usize> {
        let n = s.len();
        let mut sa: Vec<usize> = (0..n).collect();
        let mut rank: Vec<i32> = s.chars().map(|c| c as i32).collect();
        let mut k = 1;

        while k < n {
            sa.sort_by(|&a, &b| {
                if rank[a] != rank[b] {
                    rank[a].cmp(&rank[b])
                } else {
                    let ra = if a + k < n { rank[a + k] } else { -1 };
                    let rb = if b + k < n { rank[b + k] } else { -1 };
                    ra.cmp(&rb)
                }
            });

            let mut new_rank = vec![0; n];
            new_rank[sa[0]] = 0;
            for i in 1..n {
                let prev = sa[i - 1];
                let curr = sa[i];
                let same = rank[prev] == rank[curr] &&
                    (if prev + k < n { rank[prev + k] } else { -1 }) ==
                    (if curr + k < n { rank[curr + k] } else { -1 });
                new_rank[curr] = new_rank[prev] + if same { 0 } else { 1 };
            }
            rank = new_rank;
            k *= 2;
        }

        sa
    }

    pub fn get_suffix_array(&self) -> Vec<usize> {
        self.suffix_array.clone()
    }

    pub fn get_suffix(&self, index: usize) -> &str {
        &self.text[index..]
    }

    pub fn get_lcp_array(&mut self) -> Vec<usize> {
        if self.lcp_array.is_none() {
            self.lcp_array = Some(self.build_lcp_array());
        }
        self.lcp_array.as_ref().unwrap().clone()
    }

    fn build_lcp_array(&self) -> Vec<usize> {
        let n = self.text.len();
        let mut rank = vec![0; n];
        for i in 0..n {
            rank[self.suffix_array[i]] = i;
        }

        let mut lcp = vec![0; n - 1];
        let mut k = 0;
        for i in 0..n {
            if rank[i] == n - 1 {
                k = 0;
                continue;
            }
            let j = self.suffix_array[rank[i] + 1];
            while i + k < n && j + k < n && self.text.chars().nth(i + k) == self.text.chars().nth(j + k) {
                k += 1;
            }
            lcp[rank[i]] = k;
            if k > 0 {
                k -= 1;
            }
        }
        lcp
    }

    pub fn search(&self, pattern: &str) -> Vec<usize> {
        let mut result = Vec::new();
        let m = pattern.len();
        let n = self.text.len();

        let mut low = 0;
        let mut high = n - 1;

        while low <= high {
            let mid = (low + high) / 2;
            let suffix = self.get_suffix(self.suffix_array[mid]);
            let min_len = if m < suffix.len() { m } else { suffix.len() };
            let cmp = pattern[..min_len].cmp(&suffix[..min_len]);

            match cmp {
                std::cmp::Ordering::Equal => {
                    result.push(self.suffix_array[mid]);
                    let mut left = mid as isize - 1;
                    while left >= 0 {
                        let left_suffix = self.get_suffix(self.suffix_array[left as usize]);
                        if left_suffix.starts_with(pattern) {
                            result.push(self.suffix_array[left as usize]);
                            left -= 1;
                        } else {
                            break;
                        }
                    }
                    let mut right = mid + 1;
                    while right < n {
                        let right_suffix = self.get_suffix(self.suffix_array[right]);
                        if right_suffix.starts_with(pattern) {
                            result.push(self.suffix_array[right]);
                            right += 1;
                        } else {
                            break;
                        }
                    }
                    break;
                }
                std::cmp::Ordering::Less => {
                    high = mid - 1;
                }
                std::cmp::Ordering::Greater => {
                    low = mid + 1;
                }
            }
        }

        result.sort();
        result
    }

    pub fn get_longest_common_prefix(&mut self) -> usize {
        let lcp = self.get_lcp_array();
        lcp.iter().max().copied().unwrap_or(0)
    }

    pub fn get_longest_repeated_substring(&mut self) -> String {
        let lcp = self.get_lcp_array();
        let mut max_len = 0;
        let mut max_index = 0;

        for i in 0..lcp.len() {
            if lcp[i] > max_len {
                max_len = lcp[i];
                max_index = i;
            }
        }

        if max_len == 0 {
            return String::new();
        }
        self.text[self.suffix_array[max_index]..self.suffix_array[max_index] + max_len].to_string()
    }
}

#[derive(Debug, Clone)]
pub struct KDPoint {
    pub coordinates: Vec<f64>,
}

impl KDPoint {
    pub fn new(coordinates: Vec<f64>) -> Self {
        KDPoint { coordinates }
    }
}

struct KDNode {
    point: KDPoint,
    left: Option<Box<KDNode>>,
    right: Option<Box<KDNode>>,
    axis: usize,
}

pub struct KDTree {
    root: Option<Box<KDNode>>,
    dimensions: usize,
}

impl KDTree {
    pub fn new(points: Vec<KDPoint>) -> Self {
        if points.is_empty() {
            return KDTree {
                root: None,
                dimensions: 0,
            };
        }
        let dimensions = points[0].coordinates.len();
        let root = Self::build_tree(points, 0, dimensions);
        KDTree { root, dimensions }
    }

    fn build_tree(mut points: Vec<KDPoint>, depth: usize, dimensions: usize) -> Option<Box<KDNode>> {
        if points.is_empty() {
            return None;
        }

        let axis = depth % dimensions;
        points.sort_by(|a, b| a.coordinates[axis].partial_cmp(&b.coordinates[axis]).unwrap());
        let median = points.len() / 2;

        let left_points = points.drain(0..median).collect();
        let median_point = points.remove(0);
        let right_points = points;

        Some(Box::new(KDNode {
            point: median_point,
            left: Self::build_tree(left_points, depth + 1, dimensions),
            right: Self::build_tree(right_points, depth + 1, dimensions),
            axis,
        }))
    }

    pub fn insert(&mut self, point: KDPoint) {
        if self.root.is_none() {
            self.dimensions = point.coordinates.len();
            self.root = Some(Box::new(KDNode {
                point,
                left: None,
                right: None,
                axis: 0,
            }));
            return;
        }

        let mut current = self.root.as_mut().unwrap();
        let mut depth = 0;

        loop {
            let axis = depth % self.dimensions;
            if point.coordinates[axis] < current.point.coordinates[axis] {
                if current.left.is_none() {
                    current.left = Some(Box::new(KDNode {
                        point,
                        left: None,
                        right: None,
                        axis: (depth + 1) % self.dimensions,
                    }));
                    break;
                }
                current = current.left.as_mut().unwrap();
            } else {
                if current.right.is_none() {
                    current.right = Some(Box::new(KDNode {
                        point,
                        left: None,
                        right: None,
                        axis: (depth + 1) % self.dimensions,
                    }));
                    break;
                }
                current = current.right.as_mut().unwrap();
            }
            depth += 1;
        }
    }

    fn distance_squared(a: &[f64], b: &[f64]) -> f64 {
        a.iter().zip(b.iter()).map(|(x, y)| (x - y) * (x - y)).sum()
    }

    pub fn nearest_neighbor(&self, target: &[f64]) -> Option<KDPoint> {
        if self.root.is_none() || target.len() != self.dimensions {
            return None;
        }

        let mut best = None;
        let mut best_dist = f64::INFINITY;

        Self::search_nearest(
            self.root.as_ref().unwrap(),
            target,
            0,
            &mut best,
            &mut best_dist,
            self.dimensions,
        );

        best
    }

    fn search_nearest(
        node: &KDNode,
        target: &[f64],
        depth: usize,
        best: &mut Option<KDPoint>,
        best_dist: &mut f64,
        dimensions: usize,
    ) {
        let dist = Self::distance_squared(&node.point.coordinates, target);
        if dist < *best_dist {
            *best_dist = dist;
            *best = Some(node.point.clone());
        }

        let axis = depth % dimensions;
        let go_left = target[axis] < node.point.coordinates[axis];

        let (near, far) = if go_left {
            (&node.left, &node.right)
        } else {
            (&node.right, &node.left)
        };

        if let Some(near_node) = near {
            Self::search_nearest(near_node, target, depth + 1, best, best_dist, dimensions);
        }

        let plane_dist = (target[axis] - node.point.coordinates[axis]) * (target[axis] - node.point.coordinates[axis]);
        if plane_dist < *best_dist {
            if let Some(far_node) = far {
                Self::search_nearest(far_node, target, depth + 1, best, best_dist, dimensions);
            }
        }
    }

    pub fn range_search(&self, min: &[f64], max: &[f64]) -> Vec<KDPoint> {
        let mut result = Vec::new();

        if self.root.is_none() || min.len() != self.dimensions || max.len() != self.dimensions {
            return result;
        }

        Self::search_range(self.root.as_ref().unwrap(), min, max, &mut result, self.dimensions);

        result
    }

    fn search_range(
        node: &KDNode,
        min: &[f64],
        max: &[f64],
        result: &mut Vec<KDPoint>,
        dimensions: usize,
    ) {
        let point = &node.point.coordinates;
        let mut in_range = true;
        for i in 0..dimensions {
            if point[i] < min[i] || point[i] > max[i] {
                in_range = false;
                break;
            }
        }
        if in_range {
            result.push(node.point.clone());
        }

        let axis = node.axis;
        if min[axis] <= point[axis] {
            if let Some(left) = &node.left {
                Self::search_range(left, min, max, result, dimensions);
            }
        }
        if max[axis] >= point[axis] {
            if let Some(right) = &node.right {
                Self::search_range(right, min, max, result, dimensions);
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    Red,
    Black,
}

struct RBTreeNode<T: Clone + PartialOrd> {
    value: T,
    color: Color,
    left: Option<Rc<RefCell<RBTreeNode<T>>>>,
    right: Option<Rc<RefCell<RBTreeNode<T>>>>,
    parent: Option<Weak<RefCell<RBTreeNode<T>>>>,
}

pub struct RedBlackTree<T: Clone + PartialOrd> {
    root: Option<Rc<RefCell<RBTreeNode<T>>>>,
}

impl<T: Clone + PartialOrd + fmt::Display> RedBlackTree<T> {
    pub fn new() -> Self {
        RedBlackTree { root: None }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    fn get_color(node: &Option<Rc<RefCell<RBTreeNode<T>>>>) -> Color {
        node.as_ref().map_or(Color::Black, |n| n.borrow().color)
    }

    pub fn insert(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(RBTreeNode {
            value,
            color: Color::Red,
            left: None,
            right: None,
            parent: None,
        }));

        let mut parent: Option<Rc<RefCell<RBTreeNode<T>>>> = None;
        let mut current = self.root.clone();

        while let Some(node) = current {
            parent = Some(node.clone());
            if new_node.borrow().value < node.borrow().value {
                current = node.borrow().left.clone();
            } else {
                current = node.borrow().right.clone();
            }
        }

        new_node.borrow_mut().parent = parent.as_ref().map(|p| Rc::downgrade(p));

        if parent.is_none() {
            self.root = Some(new_node.clone());
        } else {
            let parent = parent.unwrap();
            if new_node.borrow().value < parent.borrow().value {
                parent.borrow_mut().left = Some(new_node.clone());
            } else {
                parent.borrow_mut().right = Some(new_node.clone());
            }
        }

        new_node.borrow_mut().color = Color::Red;
        self.insert_fixup(new_node);
    }

    fn insert_fixup(&mut self, mut z: Rc<RefCell<RBTreeNode<T>>>) {
        loop {
            let parent_opt = z.borrow().parent.clone().and_then(|w| w.upgrade());
            let parent = match parent_opt {
                Some(p) => p,
                None => break,
            };

            if parent.borrow().color != Color::Red {
                break;
            }

            let grandparent_opt = parent.borrow().parent.clone().and_then(|w| w.upgrade());
            let grandparent = match grandparent_opt {
                Some(g) => g,
                None => break,
            };

            let is_left_child = grandparent.borrow().left.as_ref().map_or(false, |p| Rc::ptr_eq(p, &parent));

            let uncle = if is_left_child {
                grandparent.borrow().right.clone()
            } else {
                grandparent.borrow().left.clone()
            };

            if Self::get_color(&uncle) == Color::Red {
                parent.borrow_mut().color = Color::Black;
                if let Some(u) = uncle {
                    u.borrow_mut().color = Color::Black;
                }
                grandparent.borrow_mut().color = Color::Red;
                z = grandparent;
            } else {
                let is_z_left = parent.borrow().left.as_ref().map_or(false, |c| Rc::ptr_eq(c, &z));

                let mut current_parent = parent.clone();

                if is_left_child {
                    if !is_z_left {
                        self.left_rotate(current_parent.clone());
                        let new_parent = current_parent.borrow().parent.clone().and_then(|w| w.upgrade()).unwrap();
                        current_parent = new_parent;
                    }
                    current_parent.borrow_mut().color = Color::Black;
                    grandparent.borrow_mut().color = Color::Red;
                    self.right_rotate(grandparent);
                } else {
                    if is_z_left {
                        self.right_rotate(current_parent.clone());
                        let new_parent = current_parent.borrow().parent.clone().and_then(|w| w.upgrade()).unwrap();
                        current_parent = new_parent;
                    }
                    current_parent.borrow_mut().color = Color::Black;
                    grandparent.borrow_mut().color = Color::Red;
                    self.left_rotate(grandparent);
                }
                break;
            }
        }

        if let Some(root) = self.root.clone() {
            root.borrow_mut().color = Color::Black;
        }
    }

    fn left_rotate(&mut self, x: Rc<RefCell<RBTreeNode<T>>>) {
        let y = x.borrow().right.clone().unwrap();
        x.borrow_mut().right = y.borrow().left.clone();

        if let Some(left) = y.borrow().left.clone() {
            left.borrow_mut().parent = Some(Rc::downgrade(&x));
        }

        y.borrow_mut().parent = x.borrow().parent.clone();

        if let Some(parent) = x.borrow().parent.clone().and_then(|w| w.upgrade()) {
            let is_left = parent.borrow().left.as_ref().map_or(false, |l| Rc::ptr_eq(l, &x));
            if is_left {
                parent.borrow_mut().left = Some(Rc::clone(&y));
            } else {
                parent.borrow_mut().right = Some(Rc::clone(&y));
            }
        } else {
            self.root = Some(Rc::clone(&y));
        }

        y.borrow_mut().left = Some(Rc::clone(&x));
        x.borrow_mut().parent = Some(Rc::downgrade(&y));
    }

    fn right_rotate(&mut self, y: Rc<RefCell<RBTreeNode<T>>>) {
        let x = y.borrow().left.clone().unwrap();
        y.borrow_mut().left = x.borrow().right.clone();

        if let Some(right) = x.borrow().right.clone() {
            right.borrow_mut().parent = Some(Rc::downgrade(&y));
        }

        x.borrow_mut().parent = y.borrow().parent.clone();

        if let Some(parent) = y.borrow().parent.clone().and_then(|w| w.upgrade()) {
            let is_right = parent.borrow().right.as_ref().map_or(false, |r| Rc::ptr_eq(r, &y));
            if is_right {
                parent.borrow_mut().right = Some(Rc::clone(&x));
            } else {
                parent.borrow_mut().left = Some(Rc::clone(&x));
            }
        } else {
            self.root = Some(Rc::clone(&x));
        }

        x.borrow_mut().right = Some(Rc::clone(&y));
        y.borrow_mut().parent = Some(Rc::downgrade(&x));
    }

    pub fn search(&self, value: T) -> bool {
        let mut current = self.root.clone();
        while let Some(node) = current {
            if value == node.borrow().value {
                return true;
            } else if value < node.borrow().value {
                current = node.borrow().left.clone();
            } else {
                current = node.borrow().right.clone();
            }
        }
        false
    }

    pub fn in_order_traversal(&self) -> Vec<T> {
        let mut result = Vec::new();
        self.in_order_helper(&self.root, &mut result);
        result
    }

    fn in_order_helper(&self, node: &Option<Rc<RefCell<RBTreeNode<T>>>>, result: &mut Vec<T>) {
        if let Some(n) = node {
            self.in_order_helper(&n.borrow().left, result);
            result.push(n.borrow().value.clone());
            self.in_order_helper(&n.borrow().right, result);
        }
    }
}

fn main() {
    println!("Advanced Data Structures Example");
}
