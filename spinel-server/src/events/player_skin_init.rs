use crate::entity::Player;
use crate::entity::player::PlayerSkin;
use spinel_macros::event_dispatcher;

#[event_dispatcher(with_client: true)]
pub struct PlayerSkinInitEvent {
    player: *mut Player,
    skin: Option<PlayerSkin>,
}

impl PlayerSkinInitEvent {
    pub fn new(player: *mut Player, skin: Option<PlayerSkin>) -> Self {
        Self {
            player,
            skin,
            connection_ptr: None,
        }
    }

    pub fn player(&mut self) -> &mut Player {
        unsafe { &mut *self.player }
    }

    pub fn skin(&self) -> Option<&PlayerSkin> {
        self.skin.as_ref()
    }

    pub fn set_skin(&mut self, skin: Option<PlayerSkin>) {
        self.skin = skin;
    }

    pub fn into_skin(self) -> Option<PlayerSkin> {
        self.skin
    }
}
