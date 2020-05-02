use core::fmt;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

// TODO - extract key and value types
pub struct OrdMap(HashMap<String, String>);

impl Clone for OrdMap {
    fn clone(&self) -> Self {
        let mut result = HashMap::new();
        for (k, v) in self.0.iter() {
            result.insert(k.clone(), v.clone());
        }
        Self(result)
    }
}

impl Default for OrdMap {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

impl PartialEq for OrdMap {
    fn eq(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }
        for (k, v) in self.0.iter() {
            if let Some(other_v) = other.0.get(k) {
                if other_v != v {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
}

impl Eq for OrdMap {}

impl OrdMap {
    pub fn map(&self) -> &HashMap<String, String> {
        return &self.0;
    }

    pub fn mut_map(&mut self) -> &mut HashMap<String, String> {
        return &mut self.0;
    }

    fn size_le(&self, other: &Self) -> bool {
        self.0.len() < other.0.len()
    }

    fn size_gt(&self, other: &Self) -> bool {
        self.0.len() > other.0.len()
    }
}

impl PartialOrd for OrdMap {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.size_le(other) {
            return Some(Ordering::Less);
        } else if self.size_gt(other) {
            return Some(Ordering::Greater);
        }
        for (k, v) in self.0.iter() {
            if let Some(other_v) = other.0.get(k.as_str()) {
                if v < other_v {
                    return Some(Ordering::Less);
                } else if v > other_v {
                    return Some(Ordering::Greater);
                }
            } else {
                // we will define the hash map that first has a value that the other one doesn't as
                // being 'larger'.
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

impl fmt::Debug for OrdMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Hash for OrdMap {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (k, v) in self.0.iter() {
            k.hash(state);
            v.hash(state);
        }
    }
}
