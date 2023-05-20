use std::collections::HashSet;


pub fn factorial(num: u64) -> u64 {
    if num == 1 as u64 {
        return 1;
    }
    else {
        return num*factorial(num-1);
    }
}

// This trait allows a collection or sequence of values check all unique elements.
pub trait UniqueElements<T> {
    fn unique(&self) -> Vec<T>;
}

/// Implementation of UniqueElements for a Vector of datatype `T`.
impl<T: Eq + std::hash::Hash + Clone + Ord> UniqueElements<T> for Vec<T> {
    fn unique(&self) -> Vec<T> {
        let set: HashSet<_> = self.iter().cloned().collect();
        let mut res: Vec<T> = set.into_iter().collect();
        res.sort();
        return res;
    }
}