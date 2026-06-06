use spinel_macros::declare_namespace;

declare_namespace!("minecraft");

pub mod entity;
pub mod network;
pub mod raycast;

pub use spinel_utils as utils;
