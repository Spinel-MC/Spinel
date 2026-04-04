use spinel_macros::declare_module_dependency;

pub mod ping_request;
pub mod status_request;

//marked for deletion (I need a cleaner way of doing this)
declare_module_dependency!(
    "server_list_ping",
    ("status", "ping", "intention", "legacy_server_list_ping")
);
