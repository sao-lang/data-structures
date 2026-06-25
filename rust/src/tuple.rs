#![allow(dead_code)]

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tuple<T: Clone> {
    items: Vec<T>,
}

impl<T: Clone> Tuple<T> {
    pub fn new(items: Vec<T>) -> Self {
        Tuple { items }
    }

    pub fn from_slice(items: &[T]) -> Self {
        Tuple {
            items: items.to_vec(),
        }
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn at(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }

    pub fn first(&self) -> Option<&T> {
        self.items.first()
    }

    pub fn last(&self) -> Option<&T> {
        self.items.last()
    }

    pub fn to_vec(&self) -> Vec<T> {
        self.items.clone()
    }

    pub fn map<U: Clone, F: Fn(&T) -> U>(&self, f: F) -> Tuple<U> {
        Tuple {
            items: self.items.iter().map(f).collect(),
        }
    }

    pub fn filter<F: Fn(&T) -> bool>(&self, f: F) -> Tuple<T> {
        Tuple {
            items: self.items.iter().filter(|&x| f(x)).cloned().collect(),
        }
    }

    pub fn reduce<F: Fn(T, &T) -> T>(&self, f: F, initial: T) -> T {
        self.items.iter().fold(initial, f)
    }

    pub fn concat(&self, other: &Tuple<T>) -> Tuple<T> {
        let mut result = self.items.clone();
        result.extend(other.items.clone());
        Tuple { items: result }
    }

    pub fn slice(&self, start: usize, end: usize) -> Tuple<T> {
        let start = start.min(self.items.len());
        let end = end.min(self.items.len());
        if start >= end {
            return Tuple { items: vec![] };
        }
        Tuple {
            items: self.items[start..end].to_vec(),
        }
    }

    pub fn take(&self, n: usize) -> Tuple<T> {
        self.slice(0, n)
    }

    pub fn drop(&self, n: usize) -> Tuple<T> {
        self.slice(n, self.items.len())
    }

    pub fn contains<F: Fn(&T) -> bool>(&self, f: F) -> bool {
        self.items.iter().any(f)
    }

    pub fn find<F: Fn(&T) -> bool>(&self, f: F) -> Option<usize> {
        self.items.iter().position(f)
    }

    pub fn count<F: Fn(&T) -> bool>(&self, f: F) -> usize {
        self.items.iter().filter(|&x| f(x)).count()
    }

    pub fn reverse(&self) -> Tuple<T> {
        let mut items = self.items.clone();
        items.reverse();
        Tuple { items }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.items.iter()
    }
}

impl<T: Clone + fmt::Display> fmt::Display for Tuple<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let items: Vec<String> = self.items.iter().map(|x| x.to_string()).collect();
        write!(f, "Tuple({})", items.join(", "))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pair<T: Clone, U: Clone> {
    first: T,
    second: U,
}

impl<T: Clone, U: Clone> Pair<T, U> {
    pub fn new(first: T, second: U) -> Self {
        Pair { first, second }
    }

    pub fn first(&self) -> &T {
        &self.first
    }

    pub fn second(&self) -> &U {
        &self.second
    }

    pub fn swap(&self) -> Pair<U, T> {
        Pair::new(self.second.clone(), self.first.clone())
    }

    pub fn to_tuple(&self) -> (T, U) {
        (self.first.clone(), self.second.clone())
    }

    pub fn map_first<F: Fn(T) -> T>(&self, f: F) -> Pair<T, U> {
        Pair::new(f(self.first.clone()), self.second.clone())
    }

    pub fn map_second<F: Fn(U) -> U>(&self, f: F) -> Pair<T, U> {
        Pair::new(self.first.clone(), f(self.second.clone()))
    }

    pub fn map_both<F1: Fn(T) -> T, F2: Fn(U) -> U>(
        &self,
        f1: F1,
        f2: F2,
    ) -> Pair<T, U> {
        Pair::new(f1(self.first.clone()), f2(self.second.clone()))
    }
}

impl<T: Clone + fmt::Display, U: Clone + fmt::Display> fmt::Display for Pair<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Pair({}, {})", self.first, self.second)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Triple<T: Clone, U: Clone, V: Clone> {
    first: T,
    second: U,
    third: V,
}

impl<T: Clone, U: Clone, V: Clone> Triple<T, U, V> {
    pub fn new(first: T, second: U, third: V) -> Self {
        Triple {
            first,
            second,
            third,
        }
    }

    pub fn first(&self) -> &T {
        &self.first
    }

    pub fn second(&self) -> &U {
        &self.second
    }

    pub fn third(&self) -> &V {
        &self.third
    }

    pub fn map_first<F: Fn(&T) -> T>(&self, f: F) -> Triple<T, U, V> {
        Triple::new(f(&self.first), self.second.clone(), self.third.clone())
    }

    pub fn map_second<F: Fn(&U) -> U>(&self, f: F) -> Triple<T, U, V> {
        Triple::new(self.first.clone(), f(&self.second), self.third.clone())
    }

    pub fn map_third<F: Fn(&V) -> V>(&self, f: F) -> Triple<T, U, V> {
        Triple::new(self.first.clone(), self.second.clone(), f(&self.third))
    }
}

impl<T: Clone + fmt::Display, U: Clone + fmt::Display, V: Clone + fmt::Display> fmt::Display
    for Triple<T, U, V>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Triple({}, {}, {})",
            self.first, self.second, self.third
        )
    }
}

pub fn zip<T: Clone, U: Clone>(t1: &Tuple<T>, t2: &Tuple<U>) -> Tuple<Pair<T, U>> {
    let min_len = t1.size().min(t2.size());
    let mut result = Vec::with_capacity(min_len);
    for i in 0..min_len {
        result.push(Pair::new(
            t1.at(i).unwrap().clone(),
            t2.at(i).unwrap().clone(),
        ));
    }
    Tuple::new(result)
}

pub fn run_tuple_examples() {
    println!("=== Tuple Example ===");
    let t1 = Tuple::new(vec![1, 2, 3, 4, 5]);
    println!("Tuple: {}", t1);
    println!("Size: {}", t1.size());
    println!("First: {:?}", t1.first());
    println!("Last: {:?}", t1.last());
    println!("At index 2: {:?}", t1.at(2));
    println!();

    println!("=== Tuple Operations ===");
    let t2 = Tuple::new(vec!["a", "b", "c"]);
    println!("Tuple t2: {}", t2);
    println!("Concat t1 + t2: {}", t1.concat(&Tuple::new(vec![])));
    println!("Slice t1[1..4]: {}", t1.slice(1, 4));
    println!("Take 3 from t1: {}", t1.take(3));
    println!("Drop 2 from t1: {}", t1.drop(2));
    println!("Reverse t1: {}", t1.reverse());
    println!("Map t1 (x * 2): {}", t1.map(|&x| x * 2));
    println!("Filter t1 (even): {}", t1.filter(|&x| x % 2 == 0));
    println!(
        "Reduce t1 (sum): {}",
        t1.reduce(|a, &b| a + b, 0)
    );
    println!();

    println!("=== Pair Example ===");
    let p = Pair::new(10, "hello");
    println!("Pair: {}", p);
    println!("First: {}", p.first());
    println!("Second: {}", p.second());
    println!("Swap: {}", p.swap());
    println!("Map first (+5): {}", p.map_first(|x| x + 5));
    println!();

    println!("=== Triple Example ===");
    let tri = Triple::new("a", 100, true);
    println!("Triple: {}", tri);
    println!("First: {}", tri.first());
    println!("Second: {}", tri.second());
    println!("Third: {}", tri.third());
    println!();

    println!("=== Zip Example ===");
    let t3 = Tuple::new(vec![1, 2, 3]);
    let t4 = Tuple::new(vec!["x", "y", "z"]);
    let zipped = zip(&t3, &t4);
    print!("Zip {} and {}: Tuple(", t3, t4);
    for (i, pair) in zipped.iter().enumerate() {
        if i > 0 {
            print!(", ");
        }
        print!("({}, {})", pair.first(), pair.second());
    }
    println!(")");
    println!();

    println!("=== Comparison Example ===");
    let t5 = Tuple::new(vec![1, 2, 3]);
    println!("Contains 2 in t5: {}", t5.contains(|&x| x == 2));
    println!("Index of 3 in t5: {:?}", t5.find(|&x| x == 3));
}

fn main() {
    run_tuple_examples();
}
