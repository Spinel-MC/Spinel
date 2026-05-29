use std::sync::atomic::{AtomicI32, Ordering};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EntityId(i32);

impl EntityId {
    pub fn next() -> Self {
        static LAST_ENTITY_ID: AtomicI32 = AtomicI32::new(0);
        Self(LAST_ENTITY_ID.fetch_add(1, Ordering::SeqCst) + 1)
    }

    pub const fn value(self) -> i32 {
        self.0
    }
}
