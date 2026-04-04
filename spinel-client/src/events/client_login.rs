use spinel_macros::event_dispatcher;

#[event_dispatcher(with_server: true)]
pub struct LoginEvent {
    pub username: String,
    pub uuid: uuid::Uuid,
}

impl LoginEvent {
    pub fn new(username: String, uuid: uuid::Uuid) -> Self {
        Self {
            username,
            uuid,
            connection_ptr: None,
        }
    }
}
