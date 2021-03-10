use hashbrown::HashMap;
use crate::trap::TrapFrame;
use super::SharedTaskHandle;

pub struct ContextTable {
    inner: HashMap<SharedTaskHandle, TrapFrame>,
}
