use crate::entity::ai::{EntityAiGroup, GoalSelector, GoalSelectorHandle, TargetSelector};

#[derive(Default)]
pub struct EntityAiGroupBuilder {
    group: EntityAiGroup,
}

impl EntityAiGroupBuilder {
    pub fn add_goal_selector(mut self, goal_selector: impl GoalSelector + 'static) -> Self {
        self.group
            .goal_selectors_mut()
            .push(GoalSelectorHandle::new(goal_selector));
        self
    }

    pub fn add_target_selector(mut self, target_selector: impl TargetSelector + 'static) -> Self {
        self.group
            .target_selectors_mut()
            .push(Box::new(target_selector));
        self
    }

    pub fn build(self) -> EntityAiGroup {
        self.group
    }
}
