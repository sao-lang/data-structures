#![allow(dead_code)]

use std::fmt;
use std::hash::Hash;
use std::ops::{BitAnd, BitOr, BitXor, Not, Add, Sub};


#[derive(Debug, PartialEq, Eq)]
pub enum MapSetError {
    InitialCapacityMustBePositive,
    IndexOutOfBounds,
}

impl fmt::Display for MapSetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MapSetError::InitialCapacityMustBePositive => write!(f, "initial capacity must be positive"),
            MapSetError::IndexOutOfBounds => write!(f, "index out of bounds"),
        }
    }
}

impl std::error::Error for MapSetError {}


pub struct HashNode<K: Eq + Hash + Clone, V: Clone> {
    pub key: K,
    pub value: V,
    pub next: Option<Box<HashNode<K, V>>>,
}


pub struct Map<K: Eq + Hash + Clone + fmt::Display, V: Clone> {
    capacity: usize,
    size: usize,
    buckets: Vec<Option<Box<HashNode<K, V>>>>,
    load_factor: f64,
}

impl<K: Eq + Hash + Clone + fmt::Display, V: Clone> Map<K, V> {
    pub fn new(initial_capacity: usize) -> Result<Self, MapSetError> {
        if initial_capacity == 0 {
            return Err(MapSetError::InitialCapacityMustBePositive);
        }
        let mut buckets = Vec::with_capacity(initial_capacity);
        for _ in 0..initial_capacity {
            buckets.push(None);
        }
        Ok(Map {
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

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let index = self.hash(key);
        let mut current = &mut self.buckets[index];

        while let Some(node) = current {
            if node.key == *key {
                return Some(&mut node.value);
            }
            current = &mut node.next;
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

    pub fn update(&mut self, other: &Map<K, V>) {
        for key in other.keys() {
            if let Some(value) = other.get(&key) {
                self.set(key, value.clone());
            }
        }
    }
}

impl<K: Eq + Hash + Clone + fmt::Display, V: Clone + fmt::Display> fmt::Display for Map<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut entries = Vec::with_capacity(self.size);
        for bucket in &self.buckets {
            let mut current = bucket;
            while let Some(node) = current {
                entries.push(format!("{}: {}", node.key, node.value));
                current = &node.next;
            }
        }
        write!(f, "{{{}}}", entries.join(", "))
    }
}

impl<K: Eq + Hash + Clone + fmt::Display, V: Clone> Add for Map<K, V> {
    type Output = Self;

    fn add(self, mut rhs: Self) -> Self::Output {
        let mut result = Map::new(std::cmp::max(self.capacity, rhs.capacity)).unwrap();
        result.update(&self);
        result.update(&rhs);
        result
    }
}

impl<K: Eq + Hash + Clone + fmt::Display, V: Clone + PartialEq> PartialEq for Map<K, V> {
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size {
            return false;
        }
        for key in self.keys() {
            match (self.get(&key), other.get(&key)) {
                (Some(v1), Some(v2)) if v1 == v2 => continue,
                _ => return false,
            }
        }
        true
    }
}

impl<K: Eq + Hash + Clone + fmt::Display, V: Clone + Eq> Eq for Map<K, V> {}

// Map Iterators

pub struct MapIter<K: Eq + Hash + Clone + fmt::Display, V: Clone> {
    map: Map<K, V>,
    keys: Vec<K>,
    index: usize,
}

impl<K: Eq + Hash + Clone + fmt::Display, V: Clone> Iterator for MapIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.keys.len() {
            return None;
        }
        let key = self.keys[self.index].clone();
        let value = self.map.get(&key).unwrap().clone();
        self.index += 1;
        Some((key, value))
    }
}

impl<K: Eq + Hash + Clone + fmt::Display, V: Clone> IntoIterator for Map<K, V> {
    type Item = (K, V);
    type IntoIter = MapIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        let keys = self.keys();
        MapIter {
            map: self,
            keys,
            index: 0,
        }
    }
}

pub struct MapRefIter<'a, K: Eq + Hash + Clone + fmt::Display, V: Clone> {
    map: &'a Map<K, V>,
    buckets_iter: std::slice::Iter<'a, Option<Box<HashNode<K, V>>>>,
    current_node: Option<&'a HashNode<K, V>>,
}

impl<'a, K: Eq + Hash + Clone + fmt::Display, V: Clone> Iterator for MapRefIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        // 检查当前节点是否有下一个节点
        if let Some(node) = self.current_node {
            let result = (&node.key, &node.value);
            self.current_node = node.next.as_ref().map(|boxed| boxed.as_ref());
            return Some(result);
        }

        // 移动到下一个非空桶
        while let Some(bucket) = self.buckets_iter.next() {
            if let Some(node) = bucket {
                let result = (&node.key, &node.value);
                self.current_node = node.next.as_ref().map(|boxed| boxed.as_ref());
                return Some(result);
            }
        }

        None
    }
}

impl<'a, K: Eq + Hash + Clone + fmt::Display, V: Clone> IntoIterator for &'a Map<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = MapRefIter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        MapRefIter {
            map: self,
            buckets_iter: self.buckets.iter(),
            current_node: None,
        }
    }
}


pub struct BitMap {
    size: usize,
    bits: Vec<u64>,
}

impl BitMap {
    pub fn new(size: usize) -> Result<Self, MapSetError> {
        if size == 0 {
            return Err(MapSetError::InitialCapacityMustBePositive);
        }
        let num_words = (size + 63) / 64;
        Ok(BitMap {
            size,
            bits: vec![0; num_words],
        })
    }

    pub fn size(&self) -> usize {
        self.size
    }

    fn get_index_and_mask(&self, bit: usize) -> Result<(usize, u64), MapSetError> {
        if bit >= self.size {
            return Err(MapSetError::IndexOutOfBounds);
        }
        let index = bit / 64;
        let mask = 1u64 << (bit % 64);
        Ok((index, mask))
    }

    pub fn set(&mut self, bit: usize) -> Result<(), MapSetError> {
        let (index, mask) = self.get_index_and_mask(bit)?;
        self.bits[index] |= mask;
        Ok(())
    }

    pub fn clear(&mut self, bit: usize) -> Result<(), MapSetError> {
        let (index, mask) = self.get_index_and_mask(bit)?;
        self.bits[index] &= !mask;
        Ok(())
    }

    pub fn toggle(&mut self, bit: usize) -> Result<(), MapSetError> {
        let (index, mask) = self.get_index_and_mask(bit)?;
        self.bits[index] ^= mask;
        Ok(())
    }

    pub fn get(&self, bit: usize) -> Result<bool, MapSetError> {
        let (index, mask) = self.get_index_and_mask(bit)?;
        Ok((self.bits[index] & mask) != 0)
    }

    pub fn set_all(&mut self) {
        for word in &mut self.bits {
            *word = u64::MAX;
        }
    }

    pub fn clear_all(&mut self) {
        for word in &mut self.bits {
            *word = 0;
        }
    }

    pub fn count_set_bits(&self) -> usize {
        self.bits.iter().map(|&word| word.count_ones() as usize).sum()
    }

    pub fn find_first_set(&self) -> Option<usize> {
        for i in 0..self.size {
            if let Ok(true) = self.get(i) {
                return Some(i);
            }
        }
        None
    }

    pub fn find_first_clear(&self) -> Option<usize> {
        for i in 0..self.size {
            if let Ok(false) = self.get(i) {
                return Some(i);
            }
        }
        None
    }
}

impl fmt::Display for BitMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut bits_str = String::with_capacity(self.size);
        for i in 0..self.size {
            if let Ok(true) = self.get(i) {
                bits_str.push('1');
            } else {
                bits_str.push('0');
            }
        }
        write!(f, "{}", bits_str)
    }
}

impl fmt::Debug for BitMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BitMap({})", self)
    }
}

impl BitAnd for BitMap {
    type Output = Result<Self, MapSetError>;

    fn bitand(self, rhs: Self) -> Self::Output {
        if self.size != rhs.size {
            return Err(MapSetError::IndexOutOfBounds);
        }
        let mut result = BitMap::new(self.size)?;
        for i in 0..self.bits.len() {
            result.bits[i] = self.bits[i] & rhs.bits[i];
        }
        Ok(result)
    }
}

impl BitOr for BitMap {
    type Output = Result<Self, MapSetError>;

    fn bitor(self, rhs: Self) -> Self::Output {
        if self.size != rhs.size {
            return Err(MapSetError::IndexOutOfBounds);
        }
        let mut result = BitMap::new(self.size)?;
        for i in 0..self.bits.len() {
            result.bits[i] = self.bits[i] | rhs.bits[i];
        }
        Ok(result)
    }
}

impl BitXor for BitMap {
    type Output = Result<Self, MapSetError>;

    fn bitxor(self, rhs: Self) -> Self::Output {
        if self.size != rhs.size {
            return Err(MapSetError::IndexOutOfBounds);
        }
        let mut result = BitMap::new(self.size)?;
        for i in 0..self.bits.len() {
            result.bits[i] = self.bits[i] ^ rhs.bits[i];
        }
        Ok(result)
    }
}

impl Not for BitMap {
    type Output = Result<Self, MapSetError>;

    fn not(self) -> Self::Output {
        let mut result = BitMap::new(self.size)?;
        for i in 0..self.bits.len() {
            result.bits[i] = !self.bits[i];
        }
        Ok(result)
    }
}

// BitMap Iterators

pub struct BitMapIter {
    bitmap: BitMap,
    index: usize,
}

impl Iterator for BitMapIter {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.bitmap.size() {
            return None;
        }
        let value = self.bitmap.get(self.index).unwrap();
        self.index += 1;
        Some(value)
    }
}

impl IntoIterator for BitMap {
    type Item = bool;
    type IntoIter = BitMapIter;

    fn into_iter(self) -> Self::IntoIter {
        BitMapIter {
            bitmap: self,
            index: 0,
        }
    }
}

pub struct BitMapRefIter<'a> {
    bitmap: &'a BitMap,
    index: usize,
}

impl<'a> Iterator for BitMapRefIter<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.bitmap.size() {
            return None;
        }
        let value = self.bitmap.get(self.index).unwrap();
        self.index += 1;
        Some(value)
    }
}

impl<'a> IntoIterator for &'a BitMap {
    type Item = bool;
    type IntoIter = BitMapRefIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        BitMapRefIter {
            bitmap: self,
            index: 0,
        }
    }
}


pub struct Set<T: Eq + Hash + Clone + fmt::Display> {
    map: Map<T, bool>,
}

impl<T: Eq + Hash + Clone + fmt::Display> Set<T> {
    pub fn new(initial_capacity: usize) -> Result<Self, MapSetError> {
        Ok(Set {
            map: Map::new(initial_capacity)?,
        })
    }

    pub fn size(&self) -> usize {
        self.map.size()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn insert(&mut self, item: T) {
        self.map.set(item, true);
    }

    pub fn remove(&mut self, item: &T) -> bool {
        self.map.delete(item)
    }

    pub fn has(&self, item: &T) -> bool {
        self.map.has(item)
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }

    pub fn items(&self) -> Vec<T> {
        self.map.keys()
    }

    pub fn union(&self, other: &Set<T>) -> Result<Set<T>, MapSetError> {
        let mut result = Set::new(std::cmp::max(self.size(), other.size()) + 1)?;
        for item in self.items() {
            result.insert(item);
        }
        for item in other.items() {
            result.insert(item);
        }
        Ok(result)
    }

    pub fn intersection(&self, other: &Set<T>) -> Result<Set<T>, MapSetError> {
        let mut result = Set::new(16)?;
        let (smaller, larger) = if self.size() <= other.size() {
            (self, other)
        } else {
            (other, self)
        };
        for item in smaller.items() {
            if larger.has(&item) {
                result.insert(item);
            }
        }
        Ok(result)
    }

    pub fn difference(&self, other: &Set<T>) -> Result<Set<T>, MapSetError> {
        let mut result = Set::new(16)?;
        for item in self.items() {
            if !other.has(&item) {
                result.insert(item);
            }
        }
        Ok(result)
    }

    pub fn symmetric_difference(&self, other: &Set<T>) -> Result<Set<T>, MapSetError> {
        let mut result = Set::new(16)?;
        for item in self.items() {
            if !other.has(&item) {
                result.insert(item);
            }
        }
        for item in other.items() {
            if !self.has(&item) {
                result.insert(item);
            }
        }
        Ok(result)
    }

    pub fn is_subset(&self, other: &Set<T>) -> bool {
        if self.size() > other.size() {
            return false;
        }
        for item in self.items() {
            if !other.has(&item) {
                return false;
            }
        }
        true
    }

    pub fn is_superset(&self, other: &Set<T>) -> bool {
        other.is_subset(self)
    }
}

impl<T: Eq + Hash + Clone + fmt::Display> fmt::Display for Set<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let items: Vec<String> = self.items().iter().map(|item| format!("{}", item)).collect();
        write!(f, "{{{}}}", items.join(", "))
    }
}

impl<T: Eq + Hash + Clone + fmt::Display> fmt::Debug for Set<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Set({})", self)
    }
}

impl<T: Eq + Hash + Clone + fmt::Display> Add for Set<T> {
    type Output = Result<Self, MapSetError>;

    fn add(self, rhs: Self) -> Self::Output {
        self.union(&rhs)
    }
}

impl<T: Eq + Hash + Clone + fmt::Display> Sub for Set<T> {
    type Output = Result<Self, MapSetError>;

    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(&rhs)
    }
}

impl<T: Eq + Hash + Clone + fmt::Display> BitAnd for Set<T> {
    type Output = Result<Self, MapSetError>;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(&rhs)
    }
}

impl<T: Eq + Hash + Clone + fmt::Display> BitOr for Set<T> {
    type Output = Result<Self, MapSetError>;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(&rhs)
    }
}

impl<T: Eq + Hash + Clone + fmt::Display> BitXor for Set<T> {
    type Output = Result<Self, MapSetError>;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(&rhs)
    }
}

impl<T: Eq + Hash + Clone + fmt::Display> PartialEq for Set<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.size() != other.size() {
            return false;
        }
        self.is_subset(other)
    }
}

impl<T: Eq + Hash + Clone + fmt::Display> Eq for Set<T> {}

// Set Iterators

pub struct SetIter<T: Eq + Hash + Clone + fmt::Display> {
    map_iter: MapIter<T, bool>,
}

impl<T: Eq + Hash + Clone + fmt::Display> Iterator for SetIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.map_iter.next().map(|(key, _)| key)
    }
}

impl<T: Eq + Hash + Clone + fmt::Display> IntoIterator for Set<T> {
    type Item = T;
    type IntoIter = SetIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        SetIter {
            map_iter: self.map.into_iter(),
        }
    }
}

pub struct SetRefIter<'a, T: Eq + Hash + Clone + fmt::Display> {
    map_iter: MapRefIter<'a, T, bool>,
}

impl<'a, T: Eq + Hash + Clone + fmt::Display> Iterator for SetRefIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.map_iter.next().map(|(key, _)| key)
    }
}

impl<'a, T: Eq + Hash + Clone + fmt::Display> IntoIterator for &'a Set<T> {
    type Item = &'a T;
    type IntoIter = SetRefIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        SetRefIter {
            map_iter: (&self.map).into_iter(),
        }
    }
}


fn main() {
    println!("{}", "=".repeat(50));
    println!("Map 示例:");
    println!("{}", "=".repeat(50));
    let mut map1 = Map::<String, i32>::new(16).unwrap();
    map1.set("one".to_string(), 1);
    map1.set("two".to_string(), 2);
    map1.set("three".to_string(), 3);
    println!("Map: {}", map1);
    println!("大小: {}", map1.size());
    println!("包含 'two': {}", map1.has(&"two".to_string()));
    if let Some(val) = map1.get(&"two".to_string()) {
        println!("'two' 的值: {}", val);
    }
    println!("键: {:?}", map1.keys());
    println!("值: {:?}", map1.values());
    
    // 测试 Map 遍历
    println!("Map 遍历:");
    for (key, value) in &map1 {
        println!("{}: {}", key, value);
    }
    println!("");

    println!("{}", "=".repeat(50));
    println!("BitMap 示例:");
    println!("{}", "=".repeat(50));
    let mut bitmap = BitMap::new(10).unwrap();
    bitmap.set(0).unwrap();
    bitmap.set(2).unwrap();
    bitmap.set(5).unwrap();
    println!("BitMap: {:?}", bitmap);
    println!("设置的位数: {}", bitmap.count_set_bits());
    println!("第 2 位: {}", bitmap.get(2).unwrap());
    bitmap.toggle(2).unwrap();
    println!("翻转第 2 位后: {:?}", bitmap);
    
    // 测试 BitMap 遍历
    println!("BitMap 遍历:");
    for (i, bit) in (&bitmap).into_iter().enumerate() {
        println!("位 {}: {}", i, bit);
    }
    println!("");

    println!("{}", "=".repeat(50));
    println!("Set 示例:");
    println!("{}", "=".repeat(50));
    let mut set1 = Set::<i32>::new(16).unwrap();
    set1.insert(1);
    set1.insert(2);
    set1.insert(3);
    let mut set2 = Set::<i32>::new(16).unwrap();
    set2.insert(3);
    set2.insert(4);
    set2.insert(5);
    println!("Set1: {:?}", set1);
    println!("Set2: {:?}", set2);
    println!("并集: {:?}", set1.union(&set2).unwrap());
    println!("交集: {:?}", set1.intersection(&set2).unwrap());
    println!("差集 (Set1 - Set2): {:?}", set1.difference(&set2).unwrap());
    println!("包含 2: {}", set1.has(&2));
    
    // 测试 Set 遍历
    println!("Set1 遍历:");
    for item in &set1 {
        println!("{} ", item);
    }
    println!("");
}
