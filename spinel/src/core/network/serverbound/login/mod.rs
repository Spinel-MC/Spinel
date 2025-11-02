use spinel_macros::declare_module_dependency;

pub mod login_start;
pub mod encryption_response;
pub mod login_acknowledge;
//TODO: Implement plugin_message.
//TODO: Implement cookie_response.
//TODO: Implement login_plugin_response.

use crate as spinel;


declare_module_dependency!("login", "intention");