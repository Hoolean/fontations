//! Internal helpers.

use core::fmt;

/// SmallVec like implementation but for fixed sized arrays whose
/// size is only known at runtime.
///
/// Used to avoid allocations when possible for small or temporary
/// arrays.
///
/// Consider taking on a smallvec dependency in the future.
#[derive(Clone)]
pub struct SmallArray<T, const N: usize>
where
    T: Copy,
{
    storage: SmallStorage<T, N>,
}

impl<T, const N: usize> SmallArray<T, N>
where
    T: Copy,
{
    pub fn new(initial_value: T, len: usize) -> Self {
        Self {
            storage: if len <= N {
                SmallStorage::Inline([initial_value; N], len)
            } else {
                SmallStorage::Heap(vec![initial_value; len])
            },
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        match &self.storage {
            SmallStorage::Inline(_, len) => *len,
            SmallStorage::Heap(vec) => vec.len(),
        }
    }

    pub fn as_slice(&self) -> &[T] {
        match &self.storage {
            SmallStorage::Inline(array, len) => &array[..*len],
            SmallStorage::Heap(vec) => vec.as_slice(),
        }
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        match &mut self.storage {
            SmallStorage::Inline(array, len) => &mut array[..*len],
            SmallStorage::Heap(vec) => vec.as_mut_slice(),
        }
    }
}

impl<T, const N: usize> fmt::Debug for SmallArray<T, N>
where
    T: Copy + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.as_slice().iter()).finish()
    }
}

#[derive(Clone)]
enum SmallStorage<T, const N: usize> {
    Inline([T; N], usize),
    Heap(Vec<T>),
}

#[cfg(test)]
mod test {
    use super::{SmallArray, SmallStorage};

    #[test]
    fn choose_inline() {
        let arr = SmallArray::<_, 4>::new(0, 4);
        assert!(matches!(arr.storage, SmallStorage::Inline(..)));
        assert_eq!(arr.len(), 4);
    }

    #[test]
    fn choose_heap() {
        let arr = SmallArray::<_, 4>::new(0, 5);
        assert!(matches!(arr.storage, SmallStorage::Heap(..)));
        assert_eq!(arr.len(), 5);
    }

    #[test]
    fn store_and_read_inline() {
        let mut arr = SmallArray::<_, 8>::new(0, 8);
        for (i, value) in arr.as_mut_slice().iter_mut().enumerate() {
            *value = i * 2;
        }
        let expected = [0, 2, 4, 6, 8, 10, 12, 14];
        assert_eq!(arr.as_slice(), &expected);
        assert_eq!(format!("{arr:?}"), format!("{expected:?}"));
    }

    #[test]
    fn store_and_read_heap() {
        let mut arr = SmallArray::<_, 4>::new(0, 8);
        for (i, value) in arr.as_mut_slice().iter_mut().enumerate() {
            *value = i * 2;
        }
        let expected = [0, 2, 4, 6, 8, 10, 12, 14];
        assert_eq!(arr.as_slice(), &expected);
        assert_eq!(format!("{arr:?}"), format!("{expected:?}"));
    }
}
