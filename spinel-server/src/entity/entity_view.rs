use crate::entity::EntityId;
use std::collections::BTreeSet;

#[derive(Clone, Debug)]
pub struct EntityView {
    entity_id: EntityId,
    manual_viewers: BTreeSet<EntityId>,
    automatic_viewers: BTreeSet<EntityId>,
    auto_viewable: bool,
    auto_viewer: bool,
    has_viewable_rule: bool,
    has_viewer_rule: bool,
}

impl EntityView {
    pub fn new(entity_id: EntityId) -> Self {
        Self {
            entity_id,
            manual_viewers: BTreeSet::new(),
            automatic_viewers: BTreeSet::new(),
            auto_viewable: true,
            auto_viewer: true,
            has_viewable_rule: false,
            has_viewer_rule: false,
        }
    }

    pub fn manual_add(&mut self, viewer_id: EntityId) -> bool {
        if viewer_id == self.entity_id {
            return false;
        }
        self.manual_viewers.insert(viewer_id)
    }

    pub fn manual_remove(&mut self, viewer_id: EntityId) -> bool {
        if viewer_id == self.entity_id {
            return false;
        }
        self.manual_viewers.remove(&viewer_id)
    }

    pub fn automatic_add(&mut self, viewer_id: EntityId) -> bool {
        if viewer_id == self.entity_id || self.manual_viewers.contains(&viewer_id) {
            return false;
        }
        self.automatic_viewers.insert(viewer_id)
    }

    pub fn automatic_remove(&mut self, viewer_id: EntityId) -> bool {
        self.automatic_viewers.remove(&viewer_id)
    }

    pub fn is_viewer(&self, viewer_id: EntityId) -> bool {
        self.manual_viewers.contains(&viewer_id) || self.automatic_viewers.contains(&viewer_id)
    }

    pub fn manual_viewers(&self) -> &BTreeSet<EntityId> {
        &self.manual_viewers
    }

    pub fn automatic_viewers(&self) -> &BTreeSet<EntityId> {
        &self.automatic_viewers
    }

    pub fn viewers(&self) -> BTreeSet<EntityId> {
        self.manual_viewers
            .union(&self.automatic_viewers)
            .copied()
            .collect()
    }

    pub fn set_auto_viewable(&mut self, auto_viewable: bool) -> bool {
        let changed = self.auto_viewable != auto_viewable;
        self.auto_viewable = auto_viewable;
        if !auto_viewable {
            self.automatic_viewers.clear();
        }
        changed
    }

    pub fn set_auto_viewer(&mut self, auto_viewer: bool) -> bool {
        let changed = self.auto_viewer != auto_viewer;
        self.auto_viewer = auto_viewer;
        changed
    }

    pub fn is_auto_viewable(&self) -> bool {
        self.auto_viewable
    }

    pub fn is_auto_viewer(&self) -> bool {
        self.auto_viewer
    }

    pub fn set_has_viewable_rule(&mut self, has_viewable_rule: bool) {
        self.has_viewable_rule = has_viewable_rule;
    }

    pub fn set_has_viewer_rule(&mut self, has_viewer_rule: bool) {
        self.has_viewer_rule = has_viewer_rule;
    }

    pub fn has_predictable_viewers(&self) -> bool {
        self.auto_viewable && !self.has_viewable_rule && self.manual_viewers.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::EntityView;
    use crate::entity::EntityId;

    #[test]
    fn entity_view_manual_add_remove_match_minestom_no_op_return_values() {
        let entity_id = EntityId::next();
        let viewer_id = EntityId::next();
        let mut entity_view = EntityView::new(entity_id);

        assert!(!entity_view.manual_add(entity_id));
        assert!(entity_view.manual_add(viewer_id));
        assert!(!entity_view.manual_add(viewer_id));
        assert!(entity_view.is_viewer(viewer_id));
        assert!(entity_view.manual_remove(viewer_id));
        assert!(!entity_view.manual_remove(viewer_id));
        assert!(!entity_view.manual_remove(entity_id));
        assert!(!entity_view.is_viewer(viewer_id));
    }

    #[test]
    fn entity_view_keeps_manual_and_automatic_viewers_separate() {
        let entity_id = EntityId::next();
        let manual_viewer_id = EntityId::next();
        let automatic_viewer_id = EntityId::next();
        let mut entity_view = EntityView::new(entity_id);

        assert!(entity_view.manual_add(manual_viewer_id));
        assert!(entity_view.automatic_add(automatic_viewer_id));
        assert!(!entity_view.automatic_add(manual_viewer_id));

        assert!(entity_view.manual_viewers().contains(&manual_viewer_id));
        assert!(
            entity_view
                .automatic_viewers()
                .contains(&automatic_viewer_id)
        );
        assert_eq!(entity_view.viewers().len(), 2);
    }

    #[test]
    fn entity_view_auto_viewable_disable_clears_automatic_viewers_only() {
        let entity_id = EntityId::next();
        let manual_viewer_id = EntityId::next();
        let automatic_viewer_id = EntityId::next();
        let mut entity_view = EntityView::new(entity_id);

        entity_view.manual_add(manual_viewer_id);
        entity_view.automatic_add(automatic_viewer_id);

        assert!(entity_view.set_auto_viewable(false));
        assert!(entity_view.manual_viewers().contains(&manual_viewer_id));
        assert!(entity_view.automatic_viewers().is_empty());
        assert!(!entity_view.is_auto_viewable());
    }

    #[test]
    fn entity_view_predictable_viewers_require_auto_no_rule_and_no_manuals() {
        let entity_id = EntityId::next();
        let viewer_id = EntityId::next();
        let mut entity_view = EntityView::new(entity_id);

        assert!(entity_view.has_predictable_viewers());
        entity_view.set_has_viewable_rule(true);
        assert!(!entity_view.has_predictable_viewers());
        entity_view.set_has_viewable_rule(false);
        entity_view.manual_add(viewer_id);
        assert!(!entity_view.has_predictable_viewers());
    }
}
