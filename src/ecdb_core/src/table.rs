use std::{
    any::Any,
    sync::{
        Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard,
        atomic::{AtomicUsize, Ordering},
    },
};

use crate::entity::EntityID;

pub struct Table {
    components: Vec<RwLock<Box<dyn Any>>>,
    entity_allocator: EntityAllocator,
}

impl Table {
    pub fn new(components: Vec<Box<dyn Any>>) -> Self {
        Self {
            components: components
                .into_iter()
                .map(|component| RwLock::new(component))
                .collect(),
            entity_allocator: EntityAllocator::new(),
        }
    }

    pub fn borrow_component(
        &self,
        component_index: usize,
    ) -> Option<RwLockReadGuard<'_, Box<dyn Any>>> {
        self.components
            .get(component_index)
            .map(|component| component.read().unwrap())
    }

    pub fn borrow_mut_component(
        &self,
        component_index: usize,
    ) -> Option<RwLockWriteGuard<'_, Box<dyn Any>>> {
        self.components
            .get(component_index)
            .map(|component| component.write().unwrap())
    }
}

struct EntityAllocator {
    last_id: AtomicUsize,
    empties: Mutex<Vec<EntityID>>,
}

impl EntityAllocator {
    fn new() -> Self {
        Self {
            last_id: AtomicUsize::new(0),
            empties: Mutex::new(Vec::new()),
        }
    }

    fn alloc(&self) -> EntityID {
        let empty = self.empties.lock().unwrap().pop();

        match empty {
            Some(empty) => empty,
            None => EntityID::new(self.last_id.fetch_add(1, Ordering::Relaxed)),
        }
    }

    fn dealloc(&self, entity_id: EntityID) {
        self.empties.lock().unwrap().push(entity_id);
    }
}
