use crate::permission::{
    Permission, PermissionCheckRequest, PermissionRemovalRequest, PermissionSet, PermissionVerifier,
};
use std::collections::HashSet;

pub trait PermissionHandler {
    fn get_permission_set(&self) -> &PermissionSet;

    fn get_permission_set_mut(&mut self) -> &mut PermissionSet;

    fn get_all_permissions(&self) -> &HashSet<Permission> {
        self.get_permission_set().get_all_permissions()
    }

    fn add_permission(&mut self, permission: Permission) {
        self.get_permission_set_mut().add_permission(permission);
    }

    fn remove_permission(&mut self, request: impl Into<PermissionRemovalRequest>) {
        self.get_permission_set_mut().remove_permission(request);
    }

    fn has_permission<'request>(
        &self,
        request: impl Into<PermissionCheckRequest<'request>>,
    ) -> bool {
        self.get_permission_set().has_permission(request)
    }

    fn has_permission_with_verifier(
        &self,
        permission_name: &str,
        verifier: &dyn PermissionVerifier,
    ) -> bool {
        self.get_permission_set()
            .has_permission_with_verifier(permission_name, verifier)
    }

    fn get_permission(&self, permission_name: &str) -> Option<&Permission> {
        self.get_permission_set().get_permission(permission_name)
    }
}
