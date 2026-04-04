use uuid::Uuid;

pub struct Player {
    pub uuid: Uuid,
    pub username: String,
}

impl Player {
    pub fn new(uuid: Uuid, username: String) -> Self {
        Self { uuid, username }
    }
}
