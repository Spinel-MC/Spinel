use crate::entity::{EntityCreature, EntityId};
use crate::world::WorldSnapshot;
use std::sync::{
    Arc, Mutex, MutexGuard,
    atomic::{AtomicU64, Ordering},
};

struct GoalSelectorHandleState {
    attached_group_id: AtomicU64,
    selector: Mutex<Box<dyn GoalSelector>>,
}

#[derive(Clone)]
pub struct GoalSelectorHandle {
    state: Arc<GoalSelectorHandleState>,
}

impl GoalSelectorHandle {
    pub fn new(selector: impl GoalSelector + 'static) -> Self {
        Self {
            state: Arc::new(GoalSelectorHandleState {
                attached_group_id: AtomicU64::new(0),
                selector: Mutex::new(Box::new(selector)),
            }),
        }
    }

    pub fn get_selector_mut(&self) -> MutexGuard<'_, Box<dyn GoalSelector>> {
        match self.state.selector.lock() {
            Ok(selector) => selector,
            Err(poisoned_selector) => poisoned_selector.into_inner(),
        }
    }

    pub fn is_same_selector(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.state, &other.state)
    }

    pub(crate) fn bind_to_group(&self, group_id: u64) {
        self.state
            .attached_group_id
            .store(group_id, Ordering::Release);
    }

    pub(crate) fn is_attached_to_group(&self, group_id: u64) -> bool {
        self.state.attached_group_id.load(Ordering::Acquire) == group_id
    }
}

pub trait GoalSelector: Send {
    fn should_start(
        &mut self,
        creature: &EntityCreature,
        world: &WorldSnapshot,
        target_selectors: &mut [Box<dyn TargetSelector>],
    ) -> bool;

    fn start(
        &mut self,
        creature: &mut EntityCreature,
        world: &WorldSnapshot,
        target_selectors: &mut [Box<dyn TargetSelector>],
    );

    fn tick(
        &mut self,
        creature: &mut EntityCreature,
        world: &WorldSnapshot,
        target_selectors: &mut [Box<dyn TargetSelector>],
        time: u64,
    );

    fn should_end(
        &mut self,
        creature: &EntityCreature,
        world: &WorldSnapshot,
        target_selectors: &mut [Box<dyn TargetSelector>],
    ) -> bool;

    fn end(
        &mut self,
        creature: &mut EntityCreature,
        world: &WorldSnapshot,
        target_selectors: &mut [Box<dyn TargetSelector>],
    );

    fn find_target(
        &mut self,
        creature: &EntityCreature,
        world: &WorldSnapshot,
        target_selectors: &mut [Box<dyn TargetSelector>],
    ) -> Option<EntityId> {
        target_selectors
            .iter_mut()
            .find_map(|selector| selector.find_target(creature, world))
    }
}

pub trait TargetSelector: Send {
    fn find_target(&mut self, creature: &EntityCreature, world: &WorldSnapshot)
    -> Option<EntityId>;
}
