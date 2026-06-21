use super::super::entity_view::EntityView;
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

    assert!(entity_view.get_manual_viewers().contains(&manual_viewer_id));
    assert!(
        entity_view
            .get_automatic_viewers()
            .contains(&automatic_viewer_id)
    );
    assert_eq!(entity_view.get_viewers().len(), 2);
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
    assert!(entity_view.get_manual_viewers().contains(&manual_viewer_id));
    assert!(entity_view.get_automatic_viewers().is_empty());
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

#[test]
fn entity_view_rule_updates_refresh_automatic_viewers_like_minestom_predicates() {
    let entity_id = EntityId::next();
    let allowed_viewer = EntityId::from_raw(2);
    let denied_viewer = EntityId::from_raw(3);
    let mut entity_view = EntityView::new(entity_id);

    entity_view.update_viewable_rule(|viewer_id| viewer_id.get_value() % 2 == 0);
    entity_view.refresh_viewable_rule([entity_id, allowed_viewer, denied_viewer]);

    assert!(entity_view.is_viewer(allowed_viewer));
    assert!(!entity_view.is_viewer(denied_viewer));
    assert!(!entity_view.has_predictable_viewers());

    entity_view.clear_viewable_rule();
    assert!(entity_view.has_predictable_viewers());

    entity_view.manual_add(denied_viewer);
    entity_view.update_viewer_rule(|target_id| target_id.get_value() == 2);
    entity_view.refresh_viewer_rule();

    assert!(entity_view.is_viewer(allowed_viewer));
    assert!(!entity_view.is_viewer(denied_viewer));
    assert!(entity_view.viewer_rule_allows(allowed_viewer));
    assert!(!entity_view.viewer_rule_allows(denied_viewer));
    entity_view.clear_viewer_rule();
    assert!(entity_view.viewer_rule_allows(denied_viewer));
}
