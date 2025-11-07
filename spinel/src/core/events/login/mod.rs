use crate as spinel;
use spinel_macros::event_dispatcher;
use uuid::Uuid;

#[event_dispatcher(event: "login", with_client: true)]
pub struct PreLoginEvent {
    pub name: String,
    pub uuid: Uuid,
    pub cancelled: bool,
    pub should_authenticate: bool,
}

impl PreLoginEvent {
    pub fn new(name: String, uuid: Uuid, should_authenticate: bool) -> Self {
        Self {
            name,
            uuid,
            cancelled: false,
            should_authenticate,
            client_ptr: None,
        }
    }
}
