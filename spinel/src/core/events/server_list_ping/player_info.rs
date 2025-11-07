use spinel_utils::component::text::TextComponent;
use uuid::Uuid;

pub struct PlayerSample {
    pub name: TextComponent,
    pub uuid: Uuid,
}

impl PlayerSample {
    pub fn new(name: TextComponent, uuid: Uuid) -> Self {
        Self { name, uuid }
    }
}
