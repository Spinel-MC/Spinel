use crate::permission::{
    Permission, PermissionCheckRequest, PermissionRemovalRequest, PermissionVerifier,
};
use std::collections::HashSet;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PermissionSet {
    permissions: HashSet<Permission>,
}

impl PermissionSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub const fn get_all_permissions(&self) -> &HashSet<Permission> {
        &self.permissions
    }

    pub fn add_permission(&mut self, permission: Permission) {
        self.permissions.insert(permission);
    }

    pub fn remove_permission(&mut self, request: impl Into<PermissionRemovalRequest>) {
        let request = request.into();

        if let Some(permission) = request.get_permission() {
            self.permissions.remove(permission);
            return;
        }

        if let Some(permission_name) = request.get_permission_name() {
            self.permissions
                .retain(|permission| permission.get_permission_name() != permission_name);
        }
    }

    pub fn has_permission<'request>(
        &self,
        request: impl Into<PermissionCheckRequest<'request>>,
    ) -> bool {
        let request = request.into();

        if let Some(permission) = request.get_permission() {
            return self.has_permission_object(permission);
        }

        let Some(permission_name) = request.get_permission_name() else {
            return false;
        };

        self.has_permission_name(permission_name, request.get_verifier())
    }

    pub fn has_permission_with_verifier(
        &self,
        permission_name: &str,
        verifier: &dyn PermissionVerifier,
    ) -> bool {
        self.has_permission(
            PermissionCheckRequest::from_name(permission_name).with_verifier(verifier),
        )
    }

    pub fn get_permission(&self, permission_name: &str) -> Option<&Permission> {
        self.permissions
            .iter()
            .find(|permission| permission.get_permission_name() == permission_name)
    }

    fn has_permission_name(
        &self,
        permission_name: &str,
        verifier: Option<&dyn PermissionVerifier>,
    ) -> bool {
        let permission = self.get_permission(permission_name);

        if let Some(verifier) = verifier {
            return permission
                .map(|permission| verifier.is_valid(permission.get_nbt_data()))
                .unwrap_or(false);
        }

        let permission = permission
            .cloned()
            .unwrap_or_else(|| Permission::new(permission_name));

        self.has_permission_object(&permission)
    }

    fn has_permission_object(&self, requested_permission: &Permission) -> bool {
        self.permissions.iter().any(|permission| {
            permission == requested_permission
                || permission_name_matches_wildcard(
                    permission.get_permission_name(),
                    requested_permission.get_permission_name(),
                )
        })
    }
}

fn permission_name_matches_wildcard(permission_pattern: &str, permission_name: &str) -> bool {
    if !permission_pattern.contains('*') {
        return false;
    }

    let pattern_parts = permission_pattern.split('*').collect::<Vec<_>>();
    let first_pattern_part = pattern_parts.first().copied().unwrap_or_default();
    let last_pattern_part = pattern_parts.last().copied().unwrap_or_default();

    if !permission_name.starts_with(first_pattern_part) {
        return false;
    }

    let mut permission_name_remainder = &permission_name[first_pattern_part.len()..];

    for pattern_part in pattern_parts.iter().copied().skip(1) {
        if pattern_part.is_empty() {
            continue;
        }

        let Some(pattern_part_index) = permission_name_remainder.find(pattern_part) else {
            return false;
        };

        permission_name_remainder =
            &permission_name_remainder[pattern_part_index + pattern_part.len()..];
    }

    permission_pattern.ends_with('*') || permission_name.ends_with(last_pattern_part)
}
