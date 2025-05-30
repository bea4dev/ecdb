use std::num::NonZeroUsize;

use crate::entity::EntityID;

/// implementation of sparse set
///
///```
/// EntityID       :  0   1   2   3
/// to_dense       : [0,  1,  x,  2]
///                   |   |       |
///                   |   |   +---+
///                   |   |   |
/// dense_elements : [0] [1] [2]
///```
#[derive(Debug)]
pub struct Component<T> {
    to_dense: Vec<Option<DenseIndex>>,
    dense_elements: Vec<T>,
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
struct DenseIndex(NonZeroUsize);

impl From<usize> for DenseIndex {
    fn from(value: usize) -> Self {
        // shift for non zero
        DenseIndex(NonZeroUsize::new(value + 1).unwrap())
    }
}

impl DenseIndex {
    fn as_index(self) -> usize {
        // shift for non zero
        self.0.get() - 1
    }
}

impl<T> Component<T> {
    pub fn new() -> Self {
        Self {
            to_dense: Vec::new(),
            dense_elements: Vec::new(),
        }
    }

    pub fn register_entity_without_value(&mut self, entity_id: EntityID) {
        if entity_id.as_usize() < self.to_dense.len() {
            self.to_dense[entity_id.as_usize()] = None;
        } else {
            debug_assert!(self.to_dense.len() == entity_id.as_usize());

            self.to_dense.push(None);
        }
    }

    pub fn register_entity_with_value(&mut self, entity_id: EntityID, value: T) {
        if entity_id.as_usize() < self.to_dense.len() {
            self.to_dense[entity_id.as_usize()] = Some(self.dense_elements.len().into());
        } else {
            debug_assert!(self.to_dense.len() == entity_id.as_usize());

            self.to_dense.push(Some(self.dense_elements.len().into()));
        }

        self.dense_elements.push(value);
    }

    pub fn remove(&mut self, entity_id: EntityID) {
        let dense_index = self.to_dense[entity_id.as_usize()];

        if let Some(dense_index) = dense_index {
            let index = dense_index.as_index();
            self.dense_elements.swap_remove(index);
        }

        self.to_dense[entity_id.as_usize()] = None;
    }

    pub fn get(&self, entity_id: EntityID) -> Option<&T> {
        self.to_dense
            .get(entity_id.as_usize())
            .cloned()
            .flatten()
            .map(|index| self.dense_elements.get(index.as_index()))
            .flatten()
    }
}
