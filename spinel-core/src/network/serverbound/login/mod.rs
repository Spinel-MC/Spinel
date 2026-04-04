use spinel_macros::declare_module_dependency;

pub mod encryption_response;
pub mod login_acknowledge;
pub mod login_start;

//Marked for deletion.
declare_module_dependency!("login", "intention");
