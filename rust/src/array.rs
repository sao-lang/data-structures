#![allow(dead_code)]

use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum ArrayError {
    CapacityMustBePositive,
    InitialCapacityMustBePositive,
    ArrayIsEmpty,
    ArrayIsFull,
    IndexOutOfBounds,
}

impl fmt::Display for ArrayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArrayError::CapacityMustBePositive => write!(f, "Capacity must be positive"),
            ArrayError::InitialCapacityMustBePositive => write!(f, "Initial capacity must be positive"),
            ArrayError::ArrayIsEmpty => write!(f, "Array is empty"),
            ArrayError::ArrayIsFull => write!(f, "Array is full"),
            ArrayError::IndexOutOfBounds => write!(f, "Index out of bounds"),
        }
    }
}

impl std::error::Error for ArrayError {}

#[derive(Debug)]
pub struct FixedArray<T: Clone + Default> {
    capacity: usize,
    length: usize,
    data: Vec<Option<T>>,
}

impl<T: Clone + Default> FixedArray<T> {
    pub fn new(capacity: usize) -> Result<Self, ArrayError> {
        if capacity == 0 {
            return Err(ArrayError::CapacityMustBePositive);
        }
        Ok(FixedArray {
            capacity,
            length: 0,
            data: vec![None; capacity],
        })
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn is_full(&self) -> bool {
        self.length == self.capacity
    }

    pub fn at(&self, index: usize) -> Result<&T, ArrayError> {
        if index >= self.length {
            return Err(ArrayError::IndexOutOfBounds);
        }
        self.data[index].as_ref().ok_or(ArrayError::IndexOutOfBounds)
    }

    pub fn at_mut(&mut self, index: usize) -> Result<&mut T, ArrayError> {
        if index >= self.length {
            return Err(ArrayError::IndexOutOfBounds);
        }
        self.data[index].as_mut().ok_or(ArrayError::IndexOutOfBounds)
    }

    pub fn set(&mut self, index: usize, value: T) -> Result<(), ArrayError> {
        if index >= self.length {
            return Err(ArrayError::IndexOutOfBounds);
        }
        self.data[index] = Some(value);
        Ok(())
    }

    pub fn push(&mut self, value: T) -> Result<(), ArrayError> {
        if self.is_full() {
            return Err(ArrayError::ArrayIsFull);
        }
        self.data[self.length] = Some(value);
        self.length += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Result<T, ArrayError> {
        if self.is_empty() {
            return Err(ArrayError::ArrayIsEmpty);
        }
        self.length -= 1;
        let value = self.data[self.length].take().ok_or(ArrayError::ArrayIsEmpty)?;
        Ok(value)
    }

    pub fn insert(&mut self, index: usize, value: T) -> Result<(), ArrayError> {
        if self.is_full() {
            return Err(ArrayError::ArrayIsFull);
        }
        if index > self.length {
            return Err(ArrayError::IndexOutOfBounds);
        }
        for i in (index..self.length).rev() {
            self.data[i + 1] = self.data[i].take();
        }
        self.data[index] = Some(value);
        self.length += 1;
        Ok(())
    }

    pub fn remove(&mut self, index: usize) -> Result<T, ArrayError> {
        if self.is_empty() {
            return Err(ArrayError::ArrayIsEmpty);
        }
        if index >= self.length {
            return Err(ArrayError::IndexOutOfBounds);
        }
        let value = self.data[index].take().ok_or(ArrayError::IndexOutOfBounds)?;
        for i in index..self.length - 1 {
            self.data[i] = self.data[i + 1].take();
        }
        self.length -= 1;
        Ok(value)
    }

    pub fn find<F>(&self, value: &T, equal: F) -> Option<usize>
    where
        F: Fn(&T, &T) -> bool,
    {
        for i in 0..self.length {
            if let Some(ref item) = self.data[i] {
                if equal(item, value) {
                    return Some(i);
                }
            }
        }
        None
    }

    pub fn to_vec(&self) -> Vec<T> {
        self.data[..self.length]
            .iter()
            .filter_map(|x| x.clone())
            .collect()
    }

    pub fn clear(&mut self) {
        self.data = vec![None; self.capacity];
        self.length = 0;
    }
}

pub struct FixedArrayIter<T: Clone + Default> {
    array: FixedArray<T>,
    index: usize,
}

impl<T: Clone + Default> Iterator for FixedArrayIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.array.length {
            return None;
        }
        let value = self.array.data[self.index].take();
        self.index += 1;
        value
    }
}

impl<T: Clone + Default> IntoIterator for FixedArray<T> {
    type Item = T;
    type IntoIter = FixedArrayIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        FixedArrayIter {
            array: self,
            index: 0,
        }
    }
}

impl<'a, T: Clone + Default> IntoIterator for &'a FixedArray<T> {
    type Item = &'a T;
    type IntoIter = FixedArrayRefIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        FixedArrayRefIter {
            array: self,
            index: 0,
        }
    }
}

pub struct FixedArrayRefIter<'a, T: Clone + Default> {
    array: &'a FixedArray<T>,
    index: usize,
}

impl<'a, T: Clone + Default> Iterator for FixedArrayRefIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.array.length {
            return None;
        }
        let value = self.array.data[self.index].as_ref();
        self.index += 1;
        value
    }
}

impl<T: Clone + Default + fmt::Display> fmt::Display for FixedArray<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let elements: Vec<String> = self.data[..self.length]
            .iter()
            .filter_map(|x| x.as_ref().map(|y| y.to_string()))
            .collect();
        write!(f, "[{}]", elements.join(", "))
    }
}

#[derive(Debug)]
pub struct DynamicArray<T: Clone + Default> {
    capacity: usize,
    length: usize,
    data: Vec<Option<T>>,
    growth_factor: usize,
}

impl<T: Clone + Default> DynamicArray<T> {
    pub fn new(initial_capacity: usize) -> Result<Self, ArrayError> {
        if initial_capacity == 0 {
            return Err(ArrayError::InitialCapacityMustBePositive);
        }
        Ok(DynamicArray {
            capacity: initial_capacity,
            length: 0,
            data: vec![None; initial_capacity],
            growth_factor: 2,
        })
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    fn resize(&mut self) {
        let new_capacity = self.capacity * self.growth_factor;
        let mut new_data = vec![None; new_capacity];
        for i in 0..self.length {
            new_data[i] = self.data[i].take();
        }
        self.data = new_data;
        self.capacity = new_capacity;
    }

    pub fn at(&self, index: usize) -> Result<&T, ArrayError> {
        if index >= self.length {
            return Err(ArrayError::IndexOutOfBounds);
        }
        self.data[index].as_ref().ok_or(ArrayError::IndexOutOfBounds)
    }

    pub fn at_mut(&mut self, index: usize) -> Result<&mut T, ArrayError> {
        if index >= self.length {
            return Err(ArrayError::IndexOutOfBounds);
        }
        self.data[index].as_mut().ok_or(ArrayError::IndexOutOfBounds)
    }

    pub fn set(&mut self, index: usize, value: T) -> Result<(), ArrayError> {
        if index >= self.length {
            return Err(ArrayError::IndexOutOfBounds);
        }
        self.data[index] = Some(value);
        Ok(())
    }

    pub fn push(&mut self, value: T) {
        if self.length >= self.capacity {
            self.resize();
        }
        self.data[self.length] = Some(value);
        self.length += 1;
    }

    pub fn pop(&mut self) -> Result<T, ArrayError> {
        if self.is_empty() {
            return Err(ArrayError::ArrayIsEmpty);
        }
        self.length -= 1;
        let value = self.data[self.length].take().ok_or(ArrayError::ArrayIsEmpty)?;
        Ok(value)
    }

    pub fn insert(&mut self, index: usize, value: T) -> Result<(), ArrayError> {
        if index > self.length {
            return Err(ArrayError::IndexOutOfBounds);
        }
        if self.length >= self.capacity {
            self.resize();
        }
        for i in (index..self.length).rev() {
            self.data[i + 1] = self.data[i].take();
        }
        self.data[index] = Some(value);
        self.length += 1;
        Ok(())
    }

    pub fn remove(&mut self, index: usize) -> Result<T, ArrayError> {
        if self.is_empty() {
            return Err(ArrayError::ArrayIsEmpty);
        }
        if index >= self.length {
            return Err(ArrayError::IndexOutOfBounds);
        }
        let value = self.data[index].take().ok_or(ArrayError::IndexOutOfBounds)?;
        for i in index..self.length - 1 {
            self.data[i] = self.data[i + 1].take();
        }
        self.length -= 1;
        Ok(value)
    }

    pub fn find<F>(&self, value: &T, equal: F) -> Option<usize>
    where
        F: Fn(&T, &T) -> bool,
    {
        for i in 0..self.length {
            if let Some(ref item) = self.data[i] {
                if equal(item, value) {
                    return Some(i);
                }
            }
        }
        None
    }

    pub fn to_vec(&self) -> Vec<T> {
        self.data[..self.length]
            .iter()
            .filter_map(|x| x.clone())
            .collect()
    }

    pub fn clear(&mut self) {
        self.data = vec![None; 10];
        self.capacity = 10;
        self.length = 0;
    }

    pub fn sort<F>(&mut self, mut less: F)
    where
        F: FnMut(&T, &T) -> bool,
    {
        let mut vec: Vec<T> = self.to_vec();
        vec.sort_by(|a, b| if less(a, b) { std::cmp::Ordering::Less } else { std::cmp::Ordering::Greater });
        for i in 0..vec.len() {
            self.data[i] = Some(vec[i].clone());
        }
    }
}

pub struct DynamicArrayIter<T: Clone + Default> {
    array: DynamicArray<T>,
    index: usize,
}

impl<T: Clone + Default> Iterator for DynamicArrayIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.array.length {
            return None;
        }
        let value = self.array.data[self.index].take();
        self.index += 1;
        value
    }
}

impl<T: Clone + Default> IntoIterator for DynamicArray<T> {
    type Item = T;
    type IntoIter = DynamicArrayIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        DynamicArrayIter {
            array: self,
            index: 0,
        }
    }
}

impl<'a, T: Clone + Default> IntoIterator for &'a DynamicArray<T> {
    type Item = &'a T;
    type IntoIter = DynamicArrayRefIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        DynamicArrayRefIter {
            array: self,
            index: 0,
        }
    }
}

pub struct DynamicArrayRefIter<'a, T: Clone + Default> {
    array: &'a DynamicArray<T>,
    index: usize,
}

impl<'a, T: Clone + Default> Iterator for DynamicArrayRefIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.array.length {
            return None;
        }
        let value = self.array.data[self.index].as_ref();
        self.index += 1;
        value
    }
}

impl<T: Clone + Default + fmt::Display> fmt::Display for DynamicArray<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let elements: Vec<String> = self.data[..self.length]
            .iter()
            .filter_map(|x| x.as_ref().map(|y| y.to_string()))
            .collect();
        write!(f, "[{}]", elements.join(", "))
    }
}

fn main() {
    println!("FixedArray Example:");
    let mut fa = FixedArray::<i32>::new(5).unwrap();
    fa.push(1).unwrap();
    fa.push(2).unwrap();
    fa.push(3).unwrap();
    println!("FixedArray: {}", fa);
    println!("Iterating with for loop:");
    for val in &fa {
        print!("{} ", val);
    }
    println!();

    println!("\nDynamicArray Example:");
    let mut da = DynamicArray::<i32>::new(2).unwrap();
    da.push(10);
    da.push(20);
    da.push(30);
    println!("DynamicArray: {}", da);
    println!("Capacity: {}", da.capacity());
    println!("Iterating with for loop:");
    for val in &da {
        print!("{} ", val);
    }
    println!();
}
