use hashbrown::HashMap;
use crate::trap::TrapFrame;
use super::SharedTaskHandle;

#[derive(Debug, Clone)]
pub struct ContextTable {
    inner: HashMap<SharedTaskHandle, TrapFrame>,
}

impl ContextTable {
    pub fn new() -> ContextTable {
        ContextTable { inner: HashMap::new() }
    }

    #[inline]
    pub fn insert(&mut self, handle: SharedTaskHandle, context: TrapFrame) {
        self.inner.insert(handle, context);
    }

    #[inline]
    pub fn remove(&mut self, handle: SharedTaskHandle) -> Option<TrapFrame> {
        self.inner.remove(&handle)
    }
}
