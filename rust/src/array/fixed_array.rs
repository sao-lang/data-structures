#![allow(dead_code)]

use std::fmt::Debug;

#[derive(Debug)]
pub struct FixedArray<T> {
    items: Vec<T>,
    capacity: usize,
}

impl<T> FixedArray<T> {
    pub fn new(capacity: usize) -> Result<FixedArray<T>, String> {
        if capacity <= 0 {
            return Err("Capacity must be a positive integer.".to_owned());
        }
        return Ok(FixedArray {
            items: Vec::with_capacity(capacity),
            capacity,
        });
    }

    fn check_index(&self, index: usize) -> Result<(), String> {
        if index >= self.items.len() {
            return Err(format!(
                "Index out of bounds: {}. Current size: {}.",
                index,
                self.items.len()
            ));
        }
        Ok(())
    }

    fn check_capacity(&self) -> Result<(), String> {
        if self.items.len() > self.capacity {
            return Err("Fixed Array is full.Cannot add more elements".to_owned());
        }
        Ok(())
    }

    pub fn append(&mut self, element: T) -> Result<(), String> {
        self.check_capacity()?;
        self.items.push(element);
        Ok(())
    }

    pub fn prepend(&mut self, element: T) -> Result<(), String> {
        self.check_capacity()?;
        self.add_at(0, element)?;
        Ok(())
    }

    pub fn add_at(&mut self, index: usize, element: T) -> Result<(), String> {
        let size = self.items.len();
        if index > size {
            return Err(format!(
                "Insertion index is out of bounds. Valid range: 0 to {}. Requested: {}",
                size, index
            ));
        }
        self.check_capacity()?;
        self.items.insert(index, element);
        Ok(())
    }

    pub fn remove(&mut self, index: usize) -> Result<T, String> {
        self.check_index(index)?;
        Ok(self.items.remove(index))
    }

    pub fn pop_front(&mut self) -> Result<(), String> {
        self.remove(0)?;
        Ok(())
    }

    pub fn pop_back(&mut self) -> Result<(), String> {
        self.remove(self.items.len())?;
        Ok(())
    }

    pub fn get(&self, index: usize) -> Result<Option<&T>, String> {
        self.check_index(index)?;
        Ok(self.items.get(index))
    }

    pub fn set(&mut self, index: usize, element: T) -> Result<(), String> {
        self.check_index(index)?;
        self.items[index] = element;
        Ok(())
    }

    pub fn get_first(&self) -> Option<&T> {
        self.items.first()
    }

    pub fn get_last(&self) -> Option<&T> {
        self.items.last()
    }

    pub fn sort(&mut self)
    where
        T: Ord,
    {
        self.items.sort();
    }

    pub fn get_size(&self) -> usize {
        self.items.len()
    }

    pub fn get_capacity(&self) -> usize {
        self.capacity
    }

    pub fn get_elements(&self) -> &Vec<T> {
        &self.items
    }
}
