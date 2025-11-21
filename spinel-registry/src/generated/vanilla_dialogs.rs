use crate::dialog::{Dialog, DialogList, DialogRegistry, ExitAction, ServerLinks};
use crate::types::Identifier;
use spinel_utils::component::text::TextComponent;
pub const CUSTOM_OPTIONS: &Dialog = &Dialog::DialogList(DialogList {
    key: Identifier::vanilla_static("custom_options"),
    button_width: 310i32,
    columns: 1i32,
    dialogs: "#minecraft:pause_screen_additions",
    exit_action: ExitAction {
        label: TextComponent::translatable("gui.back"),
        width: 200i32,
    },
    external_title: TextComponent::translatable("menu.custom_options"),
    title: TextComponent::translatable("menu.custom_options.title"),
});
pub const QUICK_ACTIONS: &Dialog = &Dialog::DialogList(DialogList {
    key: Identifier::vanilla_static("quick_actions"),
    button_width: 310i32,
    columns: 1i32,
    dialogs: "#minecraft:quick_actions",
    exit_action: ExitAction {
        label: TextComponent::translatable("gui.back"),
        width: 200i32,
    },
    external_title: TextComponent::translatable("menu.quick_actions"),
    title: TextComponent::translatable("menu.quick_actions.title"),
});
pub const SERVER_LINKS: &Dialog = &Dialog::ServerLinks(ServerLinks {
    key: Identifier::vanilla_static("server_links"),
    button_width: 310i32,
    columns: 1i32,
    exit_action: ExitAction {
        label: TextComponent::translatable("gui.back"),
        width: 200i32,
    },
    external_title: TextComponent::translatable("menu.server_links"),
    title: TextComponent::translatable("menu.server_links.title"),
});
pub fn register_dialogs(registry: &mut DialogRegistry) {
    registry.register(Identifier::vanilla_static("custom_options"), CUSTOM_OPTIONS);
    registry.register(Identifier::vanilla_static("quick_actions"), QUICK_ACTIONS);
    registry.register(Identifier::vanilla_static("server_links"), SERVER_LINKS);
}
