use std::collections::HashSet;


/// This trait allows a collection or sequence of values check all unique elements.
pub trait UniqueElements<T> {
    fn unique(&self) -> Vec<T>;
}

/// Implementation of UniqueElements for a Vector of datatype `T`.
impl<T: Eq + std::hash::Hash + Clone + Ord> UniqueElements<T> for Vec<T> {
    fn unique(&self) -> Vec<T> {
        //! Find the unique elements in a given vector.
        let set: HashSet<_> = self.iter().cloned().collect();
        let mut res: Vec<T> = set.into_iter().collect();
        res.sort();
        return res;
    }
}