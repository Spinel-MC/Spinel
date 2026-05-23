use spinel_macros::declare_module_dependency;

declare_module_dependency!(
    "server_list_ping",
    ("status", "ping", "intention", "legacy_server_list_ping")
);
