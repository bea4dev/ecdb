use std::num::NonZeroUsize;

/// impl sparse set
///
///```
/// EntityID       :      0   1   2   3
/// to_dense       :     [1,  2,  x,  3]
///                       |   |       |
///                       |   |   +---+
///                       |   |   |
/// dense_elements : [0] [1] [1] [2]
///```
///
/// `dense_element[0]` is always unused, because of DenseIndex is NonZero
#[derive(Debug)]
pub struct Component<T> {
    to_dense: Vec<Option<DenseIndex>>,
    dense_elements: Vec<T>,
}

#[derive(Debug)]
#[repr(transparent)]
pub struct DenseIndex(NonZeroUsize);
