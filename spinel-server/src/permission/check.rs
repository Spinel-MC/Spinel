use crate::permission::{Permission, PermissionVerifier};

pub struct PermissionCheckRequest<'verifier> {
    target: PermissionCheckTarget,
    verifier: Option<&'verifier dyn PermissionVerifier>,
}

enum PermissionCheckTarget {
    Permission(Permission),
    PermissionName(String),
}

impl<'verifier> PermissionCheckRequest<'verifier> {
    pub fn from(permission: Permission) -> Self {
        Self {
            target: PermissionCheckTarget::Permission(permission),
            verifier: None,
        }
    }

    pub fn from_name(permission_name: impl Into<String>) -> Self {
        Self {
            target: PermissionCheckTarget::PermissionName(permission_name.into()),
            verifier: None,
        }
    }

    pub fn with_verifier(mut self, verifier: &'verifier dyn PermissionVerifier) -> Self {
        self.verifier = Some(verifier);
        self
    }

    pub(crate) const fn get_verifier(&self) -> Option<&'verifier dyn PermissionVerifier> {
        self.verifier
    }

    pub(crate) fn get_permission(&self) -> Option<&Permission> {
        match &self.target {
            PermissionCheckTarget::Permission(permission) => Some(permission),
            PermissionCheckTarget::PermissionName(_) => None,
        }
    }

    pub(crate) fn get_permission_name(&self) -> Option<&str> {
        match &self.target {
            PermissionCheckTarget::Permission(_) => None,
            PermissionCheckTarget::PermissionName(permission_name) => Some(permission_name),
        }
    }
}

impl From<Permission> for PermissionCheckRequest<'static> {
    fn from(permission: Permission) -> Self {
        Self {
            target: PermissionCheckTarget::Permission(permission),
            verifier: None,
        }
    }
}

impl From<String> for PermissionCheckRequest<'static> {
    fn from(permission_name: String) -> Self {
        Self {
            target: PermissionCheckTarget::PermissionName(permission_name),
            verifier: None,
        }
    }
}

impl From<&str> for PermissionCheckRequest<'static> {
    fn from(permission_name: &str) -> Self {
        Self {
            target: PermissionCheckTarget::PermissionName(permission_name.to_string()),
            verifier: None,
        }
    }
}
