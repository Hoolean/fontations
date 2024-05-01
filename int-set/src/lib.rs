mod bitpage;
mod bitset;

use bitset::BitSet;
use std::ops::RangeInclusive;

/// A fast & efficient unsigned integer (u32) bit set which is invertible.
#[derive(Clone, Debug)]
pub enum IntSet<T> {
    /// Records a set of integers which are members of the set.
    Standard(BitSet<T>),

    /// Records the set of integers which are not members of the set.
    Inverted(BitSet<T>),
}

impl<T: Into<u32> + Copy + Default> IntSet<T> {
    /// Create a new empty set.
    pub fn default() -> IntSet<T> {
        IntSet::Standard(BitSet::<T>::default())
    }

    /// Create a new set which contains all integers.
    pub fn all() -> IntSet<T> {
        IntSet::Inverted(BitSet::<T>::default())
    }

    /// Return the inverted version of this set.
    pub fn inverted(self) -> IntSet<T> {
        match self {
            IntSet::<T>::Standard(s) => IntSet::<T>::Inverted(s),
            IntSet::<T>::Inverted(s) => IntSet::<T>::Standard(s),
        }
    }

    /// Return a new version of this set with all members removed.
    pub fn clear(mut self) -> IntSet<T> {
        self.clear_internal_set();
        match self {
            IntSet::<T>::Standard(s) => IntSet::<T>::Standard(s),
            IntSet::<T>::Inverted(s) => IntSet::<T>::Standard(s),
        }
    }

    /// Add val as a member of this set.
    pub fn insert(&mut self, val: T) -> bool {
        match self {
            IntSet::<T>::Standard(s) => s.insert(val),
            IntSet::<T>::Inverted(s) => s.remove(val),
        }
    }

    /// Add all values in range as members of this set.
    pub fn insert_range(&mut self, range: RangeInclusive<T>) {
        match self {
            IntSet::<T>::Standard(s) => s.insert_range(range),
            IntSet::<T>::Inverted(_) => todo!("implement bitset::remove_range and call here."),
        }
    }

    /// Remove val from this set.
    pub fn remove(&mut self, val: T) -> bool {
        match self {
            IntSet::<T>::Standard(s) => s.remove(val),
            IntSet::<T>::Inverted(s) => s.insert(val),
        }
    }

    /// Returns true if val is a member of this set.
    pub fn contains(&self, val: T) -> bool {
        match self {
            IntSet::<T>::Standard(s) => s.contains(val),
            IntSet::<T>::Inverted(s) => !s.contains(val),
        }
    }

    fn clear_internal_set(&mut self) {
        match self {
            IntSet::<T>::Standard(s) => s.clear(),
            IntSet::<T>::Inverted(s) => s.clear(),
        }
    }
}

impl<T> IntSet<T> {
    /// Returns the number of members in this set.
    pub fn len(&self) -> usize {
        match self {
            IntSet::<T>::Standard(s) => s.len(),
            IntSet::<T>::Inverted(s) => u32::MAX as usize - s.len(),
        }
    }

    /// Return true if there are no members in this set.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_empty() {
        let mut set = IntSet::<u32>::default();

        assert!(set.is_empty());
        set.insert(13);
        set.insert(800);
        assert!(!set.is_empty());

        let set = set.inverted();
        assert!(!set.is_empty());

        let empty = IntSet::<u32>::default();
        assert!(empty.is_empty());
        let all = empty.inverted();
        assert!(!all.is_empty());
    }

    #[test]
    fn clear() {
        let mut set = IntSet::<u32>::default();
        set.insert(13);
        set.insert(800);

        let mut set_inverted = IntSet::<u32>::default();
        set_inverted.insert(13);
        set_inverted.insert(800);
        let set_inverted = set_inverted.inverted();

        let set = set.clear();
        assert!(set.is_empty());
        let set_inverted = set_inverted.clear();
        assert!(set_inverted.is_empty());
    }

    #[test]
    fn inverted() {
        let mut set = IntSet::<u32>::default();

        set.insert(13);
        set.insert(800);
        assert!(set.contains(13));
        assert!(set.contains(800));
        assert_eq!(set.len(), 2);

        set = set.inverted();
        assert_eq!(set.len(), u32::MAX as usize - 2);
        assert!(!set.contains(13));
        assert!(set.contains(80));
        assert!(!set.contains(800));

        set.remove(80);
        assert!(!set.contains(80));

        set.insert(13);
        assert!(set.contains(13));

        set = set.inverted();
        assert!(set.contains(80));
        assert!(set.contains(800));
    }
}