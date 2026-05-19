use crate::{DynamicRegistry, RegistryKey};
use crate::dialog::Dialog;
impl Dialog {
    pub const CUSTOM_OPTIONS: RegistryKey<Self> = RegistryKey::vanilla_static("custom_options");
    pub const QUICK_ACTIONS: RegistryKey<Self> = RegistryKey::vanilla_static("quick_actions");
    pub const SERVER_LINKS: RegistryKey<Self> = RegistryKey::vanilla_static("server_links");
}
pub fn register_dialogs(registry: &mut DynamicRegistry<Dialog>) {
    let _ = registry.register_vanilla(Dialog::CUSTOM_OPTIONS, Dialog::default());
    let _ = registry.register_vanilla(Dialog::QUICK_ACTIONS, Dialog::default());
    let _ = registry.register_vanilla(Dialog::SERVER_LINKS, Dialog::default());
}
