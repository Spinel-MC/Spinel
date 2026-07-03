use crate::entity::{Entity, Player};
use crate::permission::{Permission, PermissionHandler, PermissionSet};
use spinel_nbt::{Nbt, NbtCompound};
use spinel_registry::EntityType;
use std::net::SocketAddr;
use uuid::Uuid;

#[derive(Default)]
struct PermissionOwner {
    permissions: PermissionSet,
}

impl PermissionHandler for PermissionOwner {
    fn get_permission_set(&self) -> &PermissionSet {
        &self.permissions
    }

    fn get_permission_set_mut(&mut self) -> &mut PermissionSet {
        &mut self.permissions
    }
}

#[test]
fn permission_owner_starts_without_permissions() {
    let permission_owner = PermissionOwner::default();

    assert!(!permission_owner.has_permission(""));
    assert!(!permission_owner.has_permission("random.permission"));
}

#[test]
fn permission_owner_adds_gets_and_removes_permissions() {
    let mut permission_owner = PermissionOwner::default();
    let first_permission = Permission::new("perm.first");
    let second_permission = Permission::new("perm.second");

    permission_owner.add_permission(first_permission.clone());
    permission_owner.add_permission(second_permission.clone());

    assert_eq!(permission_owner.get_all_permissions().len(), 2);
    assert_eq!(
        permission_owner.get_permission("perm.first"),
        Some(&first_permission)
    );
    assert!(permission_owner.has_permission(first_permission.clone()));
    assert!(permission_owner.has_permission("perm.first"));

    permission_owner.remove_permission(first_permission.clone());
    assert!(!permission_owner.has_permission(first_permission));

    permission_owner.remove_permission("perm.second");
    assert!(!permission_owner.has_permission(second_permission));
}

#[test]
fn permission_owner_verifies_exact_named_nbt_permission() {
    let mut permission_owner = PermissionOwner::default();
    let nbt_data = NbtCompound::new()
        .put("name", Nbt::String("Minestom".to_string()))
        .put("amount", Nbt::Int(5));
    let permission = Permission::from_nbt_data("perm.name", nbt_data);
    let valid_nbt_data = |nbt_data: Option<&NbtCompound>| {
        nbt_data
            .and_then(|nbt_data| nbt_data.get("name"))
            .is_some_and(|name| name == &Nbt::String("Minestom".to_string()))
    };
    let invalid_nbt_data = |nbt_data: Option<&NbtCompound>| nbt_data.is_some();

    permission_owner.add_permission(permission);
    permission_owner.add_permission(Permission::new("perm.name2"));

    assert!(permission_owner.has_permission("perm.name"));
    assert!(permission_owner.has_permission_with_verifier("perm.name", &valid_nbt_data));
    assert!(!permission_owner.has_permission_with_verifier("perm.name2", &invalid_nbt_data));
    assert!(!permission_owner.has_permission_with_verifier("perm.missing", &valid_nbt_data));
}

#[test]
fn permission_owner_matches_minestom_wildcard_patterns() {
    let mut permission_owner = PermissionOwner::default();

    permission_owner.add_permission(Permission::new("foo.b*r.baz"));
    assert!(permission_owner.has_permission(Permission::new("foo.baaar.baz")));
    assert!(permission_owner.has_permission(Permission::new("foo.br.baz")));
    assert!(permission_owner.has_permission("foo.baaar.baz"));
    assert!(permission_owner.has_permission("foo.br.baz"));
    assert!(!permission_owner.has_permission(Permission::new("foo.br.bz")));
    assert!(!permission_owner.has_permission(Permission::new("foo.b.baz")));

    permission_owner.add_permission(Permission::new("foo.b*"));
    assert!(permission_owner.has_permission(Permission::new("foo.baaar.baz")));
    assert!(permission_owner.has_permission(Permission::new("foo.b")));
    assert!(permission_owner.has_permission("foo.baaar.baz"));
    assert!(permission_owner.has_permission("foo.b"));
    assert!(!permission_owner.has_permission(Permission::new("foo.")));
    assert!(!permission_owner.has_permission(Permission::new("foo/b")));

    permission_owner.add_permission(Permission::new("*"));
    assert!(permission_owner.has_permission(Permission::new("foo.bar.baz")));
}

#[test]
fn entity_and_player_expose_permission_handler_api() {
    let mut entity = Entity::new(EntityType::ZOMBIE);
    entity.add_permission(Permission::new("entity.permission"));

    assert!(entity.has_permission("entity.permission"));

    let mut player = Player::new(
        Uuid::new_v4(),
        "PermissionUser".to_string(),
        0,
        SocketAddr::from(([127, 0, 0, 1], 25565)),
    );
    player.add_permission(Permission::new("player.permission"));

    assert!(player.has_permission("player.permission"));
}
