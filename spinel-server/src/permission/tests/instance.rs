use crate::permission::Permission;
use spinel_nbt::{Nbt, NbtCompound};

#[test]
fn permission_without_nbt_data_has_only_name() {
    let permission = Permission::new("perm.name");

    assert_eq!(permission.get_permission_name(), "perm.name");
    assert_eq!(permission.get_nbt_data(), None);
}

#[test]
fn permission_from_nbt_data_compares_name_and_nbt_data() {
    let nbt_data = NbtCompound::new().put("name", Nbt::String("Spinel".to_string()));
    let permission = Permission::from_nbt_data("perm.name", nbt_data.clone());
    let same_permission = Permission::from_nbt_data("perm.name", nbt_data.clone());
    let different_nbt_permission =
        Permission::from_nbt_data("perm.name", nbt_data.clone().put("amount", Nbt::Int(5)));
    let different_name_permission = Permission::new("perm.other");

    assert_eq!(permission.get_permission_name(), "perm.name");
    assert_eq!(permission.get_nbt_data(), Some(&nbt_data));
    assert_eq!(permission, same_permission);
    assert_ne!(permission, different_nbt_permission);
    assert_ne!(permission, different_name_permission);
}
