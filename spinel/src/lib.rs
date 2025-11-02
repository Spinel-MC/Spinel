pub mod core;
pub mod util;

pub use spinel_macros;
pub use spinel_events as events;
use spinel_macros::declare_namespace;
pub use spinel_network as network;
pub use spinel_utils as utils;

#[allow(unused_imports)]
use crate as spinel;

#[doc(hidden)]
pub mod internal {
    pub use spinel_events::{ListenerFn, RegisteredEvent, RegisteredModule, RegisteredListener, RegisteredModuleDependency};
    pub use inventory;
    pub use spinel_network::{Client, ConnectionState, PacketListener, encoder, server};
}

declare_namespace!("minecraft");