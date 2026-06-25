#![allow(dead_code)]

use std::fmt;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq)]
pub enum HashTableError {
    InitialCapacityMustBePositive,
}

impl fmt::Display for HashTableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HashTableError::InitialCapacityMustBePositive => write!(f, "initial capacity must be positive"),
        }
    }
}

impl std::error::Error for HashTableError {}

struct HashNode<K: Eq + Hash + Clone, V: Clone> {
    key: K,
    value: V,
    next: Option<Box<HashNode<K, V>>>,
}

pub struct HashTable<K: Eq + Hash + Clone, V: Clone> {
    capacity: usize,
    size: usize,
    buckets: Vec<Option<Box<HashNode<K, V>>>>,
    load_factor: f64,
}

impl<K: Eq + Hash + Clone + fmt::Display, V: Clone> HashTable<K, V> {
    pub fn new(initial_capacity: usize) -> Result<Self, HashTableError> {
        if initial_capacity == 0 {
            return Err(HashTableError::InitialCapacityMustBePositive);
        }
        let mut buckets = Vec::with_capacity(initial_capacity);
        for _ in 0..initial_capacity {
            buckets.push(None);
        }
        Ok(HashTable {
            capacity: initial_capacity,
            size: 0,
            buckets,
            load_factor: 0.7,
        })
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    fn hash(&self, key: &K) -> usize {
        let key_str = format!("{}", key);
        let mut hash = 0;
        for byte in key_str.bytes() {
            hash = (hash << 5) - hash + byte as i32;
            hash = hash & hash;
        }
        hash.abs() as usize % self.capacity
    }

    fn resize(&mut self) {
        let old_buckets = std::mem::replace(&mut self.buckets, vec![]);
        self.capacity *= 2;
        self.size = 0;
        let mut new_buckets = Vec::with_capacity(self.capacity);
        for _ in 0..self.capacity {
            new_buckets.push(None);
        }
        self.buckets = new_buckets;

        for mut bucket in old_buckets {
            while let Some(mut node) = bucket {
                let next = node.next.take();
                self.set(node.key.clone(), node.value.clone());
                bucket = next;
            }
        }
    }

    pub fn set(&mut self, key: K, value: V) {
        if self.size as f64 / self.capacity as f64 >= self.load_factor {
            self.resize();
        }

        let index = self.hash(&key);
        let mut current = &mut self.buckets[index];

        while let Some(node) = current {
            if node.key == key {
                node.value = value;
                return;
            }
            current = &mut node.next;
        }

        let new_node = Box::new(HashNode {
            key,
            value,
            next: self.buckets[index].take(),
        });
        self.buckets[index] = Some(new_node);
        self.size += 1;
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let index = self.hash(key);
        let mut current = &self.buckets[index];

        while let Some(node) = current {
            if node.key == *key {
                return Some(&node.value);
            }
            current = &node.next;
        }

        None
    }

    pub fn has(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    pub fn delete(&mut self, key: &K) -> bool {
        let index = self.hash(key);
        let mut current = &mut self.buckets[index];

        loop {
            match current {
                None => return false,
                Some(node) if node.key == *key => {
                    let next = node.next.take();
                    *current = next;
                    self.size -= 1;
                    return true;
                }
                Some(node) => {
                    current = &mut node.next;
                }
            }
        }
    }

    pub fn keys(&self) -> Vec<K> {
        let mut keys = Vec::with_capacity(self.size);
        for bucket in &self.buckets {
            let mut current = bucket;
            while let Some(node) = current {
                keys.push(node.key.clone());
                current = &node.next;
            }
        }
        keys
    }

    pub fn values(&self) -> Vec<V> {
        let mut values = Vec::with_capacity(self.size);
        for bucket in &self.buckets {
            let mut current = bucket;
            while let Some(node) = current {
                values.push(node.value.clone());
                current = &node.next;
            }
        }
        values
    }

    pub fn clear(&mut self) {
        let mut new_buckets = Vec::with_capacity(self.capacity);
        for _ in 0..self.capacity {
            new_buckets.push(None);
        }
        self.buckets = new_buckets;
        self.size = 0;
    }
}

fn main() {
    println!("Hash Table Example:");
    let mut ht = HashTable::<String, i32>::new(16).unwrap();
    ht.set("one".to_string(), 1);
    ht.set("two".to_string(), 2);
    ht.set("three".to_string(), 3);
    println!("Size: {}", ht.size());
    if let Some(val) = ht.get(&"two".to_string()) {
        println!("Value for 'two': {}", val);
    }
    println!("Keys: {:?}", ht.keys());
    println!("Values: {:?}", ht.values());
    println!("Has 'three': {}", ht.has(&"three".to_string()));
    ht.delete(&"two".to_string());
    println!("Size after delete: {}", ht.size());
}
