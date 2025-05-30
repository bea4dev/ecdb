
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct EntityID(usize);

impl EntityID {
    pub(crate) fn new(id: usize) -> Self {
        Self(id)
    }

    pub(crate) fn as_usize(self) -> usize {
        self.0
    }
}
