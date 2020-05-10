use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[derive(Debug, Eq)]
pub struct Stack<T>(Vec<T>)
    where T: Clone + Eq + PartialEq + PartialOrd + Hash;

impl<T> Stack<T>
    where T: Clone + Eq + PartialEq + PartialOrd + Hash
{
    pub fn new() -> Self {
        Stack(Vec::new())
    }

    pub fn from(data: Vec<T>) -> Self {
        Stack(data)
    }

    pub fn push(&mut self, value: T) {
        self.0.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.0.is_empty() {
            return None;
        }
        Some(self.0.remove(self.0.len() - 1))
    }

    pub fn peek(&self) -> Option<&T> {
        if self.0.is_empty() {
            return None;
        }
        self.0.get(self.0.len() - 1)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        if self.0.is_empty() {
            return None;
        }
        let i = self.0.len() - 1;
        self.0.get_mut(i)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn vec(&self) -> &Vec<T> {
        &self.0
    }

    pub fn vec_mut(&mut self) -> &mut Vec<T> {
        &mut self.0
    }
}


impl<T: Clone> Clone for Stack<T>
    where T: Clone + Eq + PartialEq + PartialOrd + Hash
{
    fn clone(&self) -> Self {
        let mut result: Vec<T> = Vec::new();
        for item in self.0.iter() {
            result.push(item.clone())
        }
        Self(result)
    }
}

impl<T> Default for Stack<T>
    where T: Clone + Eq + PartialEq + PartialOrd + Hash
{
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl<T: Eq> PartialEq for Stack<T>
    where T: Clone + Eq + PartialEq + PartialOrd + Hash
{
    fn eq(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }
        for i in 0..self.0.len() {
            if self.0.get(i).unwrap().eq(other.0.get(i).unwrap()) {
                return false;
            }
        }
        true
    }
}

impl<T> Stack<T>
    where T: Clone + Eq + PartialEq + PartialOrd + Hash
{
    fn size_le(&self, other: &Self) -> bool {
        self.0.len() < other.0.len()
    }

    fn size_gt(&self, other: &Self) -> bool {
        self.0.len() > other.0.len()
    }
}

impl<T> PartialOrd for Stack<T>
    where T: Clone + Eq + PartialEq + PartialOrd + Hash
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.size_le(other) {
            return Some(Ordering::Less);
        } else if self.size_gt(other) {
            return Some(Ordering::Greater);
        }
        for i in 0..self.0.len() {
            if self.0.get(i).unwrap() < other.0.get(i).unwrap() {
                return Some(Ordering::Less);
            } else if self.0.get(i).unwrap() > other.0.get(i).unwrap() {
                return Some(Ordering::Greater);
            }
        }
        Some(Ordering::Equal)
    }

    fn lt(&self, other: &Self) -> bool {
        if let Some(ordering) = self.partial_cmp(other) {
            return ordering == Ordering::Less;
        }
        false
    }

    fn le(&self, other: &Self) -> bool {
        if let Some(ordering) = self.partial_cmp(other) {
            return ordering == Ordering::Less || ordering == Ordering::Equal;
        }
        false
    }

    fn gt(&self, other: &Self) -> bool {
        if let Some(ordering) = self.partial_cmp(other) {
            return ordering == Ordering::Greater;
        }
        false
    }

    fn ge(&self, other: &Self) -> bool {
        if let Some(ordering) = self.partial_cmp(other) {
            return ordering == Ordering::Greater || ordering == Ordering::Equal;
        }
        false
    }
}

impl<T> Hash for Stack<T>
    where T: Clone + Eq + PartialEq + PartialOrd + Hash
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        for item in self.0.iter() {
            item.hash(state);
        }
    }
}
