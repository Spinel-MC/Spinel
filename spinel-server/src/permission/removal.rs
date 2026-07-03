use crate::permission::Permission;

pub struct PermissionRemovalRequest {
    target: PermissionRemovalTarget,
}

enum PermissionRemovalTarget {
    Permission(Permission),
    PermissionName(String),
}

impl PermissionRemovalRequest {
    pub fn from(permission: Permission) -> Self {
        Self {
            target: PermissionRemovalTarget::Permission(permission),
        }
    }

    pub fn from_name(permission_name: impl Into<String>) -> Self {
        Self {
            target: PermissionRemovalTarget::PermissionName(permission_name.into()),
        }
    }

    pub(crate) fn get_permission(&self) -> Option<&Permission> {
        match &self.target {
            PermissionRemovalTarget::Permission(permission) => Some(permission),
            PermissionRemovalTarget::PermissionName(_) => None,
        }
    }

    pub(crate) fn get_permission_name(&self) -> Option<&str> {
        match &self.target {
            PermissionRemovalTarget::Permission(_) => None,
            PermissionRemovalTarget::PermissionName(permission_name) => Some(permission_name),
        }
    }
}

impl From<Permission> for PermissionRemovalRequest {
    fn from(permission: Permission) -> Self {
        Self {
            target: PermissionRemovalTarget::Permission(permission),
        }
    }
}

impl From<String> for PermissionRemovalRequest {
    fn from(permission_name: String) -> Self {
        Self {
            target: PermissionRemovalTarget::PermissionName(permission_name),
        }
    }
}

impl From<&str> for PermissionRemovalRequest {
    fn from(permission_name: &str) -> Self {
        Self {
            target: PermissionRemovalTarget::PermissionName(permission_name.to_string()),
        }
    }
}
