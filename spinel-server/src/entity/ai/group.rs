use crate::entity::EntityCreature;
use crate::entity::ai::{GoalSelectorHandle, TargetSelector};
use crate::world::WorldSnapshot;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicU64, Ordering};

pub struct EntityAiGroup {
    id: u64,
    current_goal_selector: Option<GoalSelectorHandle>,
    goal_selectors: Vec<GoalSelectorHandle>,
    target_selectors: Vec<Box<dyn TargetSelector>>,
}

pub struct GoalSelectorCollectionMut<'group> {
    group_id: u64,
    goal_selectors: &'group mut Vec<GoalSelectorHandle>,
}

static NEXT_AI_GROUP_ID: AtomicU64 = AtomicU64::new(0);

impl Default for EntityAiGroup {
    fn default() -> Self {
        Self {
            id: NEXT_AI_GROUP_ID.fetch_add(1, Ordering::Relaxed) + 1,
            current_goal_selector: None,
            goal_selectors: Vec::new(),
            target_selectors: Vec::new(),
        }
    }
}

impl Deref for GoalSelectorCollectionMut<'_> {
    type Target = Vec<GoalSelectorHandle>;

    fn deref(&self) -> &Self::Target {
        self.goal_selectors
    }
}

impl DerefMut for GoalSelectorCollectionMut<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.goal_selectors
    }
}

impl Drop for GoalSelectorCollectionMut<'_> {
    fn drop(&mut self) {
        self.goal_selectors
            .iter()
            .for_each(|goal_selector| goal_selector.bind_to_group(self.group_id));
    }
}

impl EntityAiGroup {
    pub fn get_goal_selectors(&self) -> &[GoalSelectorHandle] {
        &self.goal_selectors
    }

    pub fn get_goal_selectors_mut(&mut self) -> GoalSelectorCollectionMut<'_> {
        GoalSelectorCollectionMut {
            group_id: self.id,
            goal_selectors: &mut self.goal_selectors,
        }
    }

    pub fn get_target_selectors(&self) -> &[Box<dyn TargetSelector>] {
        &self.target_selectors
    }

    pub fn get_target_selectors_mut(&mut self) -> &mut Vec<Box<dyn TargetSelector>> {
        &mut self.target_selectors
    }

    pub fn get_current_goal_selector(&self) -> Option<&GoalSelectorHandle> {
        self.current_goal_selector.as_ref()
    }

    pub fn set_current_goal_selector(&mut self, goal_selector: Option<GoalSelectorHandle>) -> bool {
        if goal_selector
            .as_ref()
            .is_some_and(|selector| !selector.is_attached_to_group(self.id))
        {
            return false;
        }
        self.current_goal_selector = goal_selector;
        true
    }

    pub fn tick(&mut self, creature: &mut EntityCreature, world: &WorldSnapshot, time: u64) {
        let current_should_end = self.current_goal_selector.as_ref().is_some_and(|selector| {
            selector
                .get_selector_mut()
                .should_end(creature, world, &mut self.target_selectors)
        });
        if current_should_end {
            if let Some(current_goal_selector) = self.current_goal_selector.take() {
                current_goal_selector.get_selector_mut().end(
                    creature,
                    world,
                    &mut self.target_selectors,
                );
            }
        }

        for goal_selector in &self.goal_selectors {
            let is_current_goal_selector = self
                .current_goal_selector
                .as_ref()
                .is_some_and(|current| current.is_same_selector(goal_selector));
            if is_current_goal_selector {
                break;
            }
            if !goal_selector.get_selector_mut().should_start(
                creature,
                world,
                &mut self.target_selectors,
            ) {
                continue;
            }
            if let Some(current_goal_selector) = self.current_goal_selector.take() {
                current_goal_selector.get_selector_mut().end(
                    creature,
                    world,
                    &mut self.target_selectors,
                );
            }
            self.current_goal_selector = Some(goal_selector.clone());
            goal_selector
                .get_selector_mut()
                .start(creature, world, &mut self.target_selectors);
            break;
        }

        if let Some(current_goal_selector) = &self.current_goal_selector {
            current_goal_selector.get_selector_mut().tick(
                creature,
                world,
                &mut self.target_selectors,
                time,
            );
        }
    }
}
