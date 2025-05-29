use std::sync::Arc;

use crate::component::ComponentPool;

#[derive(Debug)]
pub struct Database<T: ComponentPool> {
    pool: Arc<T>,
}
