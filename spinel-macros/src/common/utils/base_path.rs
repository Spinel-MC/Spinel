use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

pub fn get_base_path(module: &str) -> TokenStream2 {
    let package_name = std::env::var("CARGO_PKG_NAME").unwrap_or_default();
    let is_internal_package = package_name == "spinel" || package_name.starts_with("spinel-");

    if !is_internal_package {
        return external_base_path(module);
    }

    internal_base_path(&package_name, module)
}

pub fn resolve_id(raw_id: &str, default_namespace: &str) -> String {
    if raw_id.contains(':') {
        return raw_id.to_string();
    }

    format!("{}:{}", default_namespace, raw_id)
}

fn external_base_path(module: &str) -> TokenStream2 {
    match module {
        "events" => quote!(::spinel::events),
        "network" => quote!(::spinel::network),
        "core" => quote!(::spinel::core),
        "server" => quote!(::spinel::server),
        "client" => quote!(::spinel::client),
        "nbt" => quote!(::spinel::nbt),
        "utils" => quote!(::spinel::utils),
        _ => quote!(::spinel),
    }
}

fn internal_base_path(package_name: &str, module: &str) -> TokenStream2 {
    match (package_name, module) {
        ("spinel-server", "server") => quote!(crate),
        ("spinel-client", "client") => quote!(crate),
        ("spinel-network", "network") => quote!(crate),
        ("spinel-events", "events") => quote!(crate),
        ("spinel-core", "core") => quote!(crate),
        ("spinel-utils", "utils") => quote!(crate),
        ("spinel-nbt", "nbt") => quote!(crate),
        (_, "events") => quote!(::spinel_events),
        (_, "network") => quote!(::spinel_network),
        (_, "core") => quote!(::spinel_core),
        (_, "server") => quote!(::spinel_server),
        (_, "client") => quote!(::spinel_client),
        (_, "nbt") => quote!(::spinel_nbt),
        (_, "utils") => quote!(::spinel_utils),
        _ => quote!(::spinel),
    }
}
