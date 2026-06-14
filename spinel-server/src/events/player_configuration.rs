use crate::entity::Player;
use spinel_macros::event_dispatcher;
use spinel_network::types::Identifier;
use uuid::Uuid;

#[event_dispatcher(is_async: true, with_client: true)]
pub struct AsyncPlayerConfigurationEvent {
    player: Player,
    is_first_config: bool,
    feature_flags: Vec<Identifier>,
    is_hardcore: bool,
    should_clear_chat: bool,
    should_send_registry_data: bool,
    spawning_world: Option<Uuid>,
}

impl AsyncPlayerConfigurationEvent {
    pub fn new(player: Player, is_first_config: bool) -> Self {
        Self {
            player,
            is_first_config,
            feature_flags: vec![Identifier::minecraft("vanilla")],
            is_hardcore: false,
            should_clear_chat: false,
            should_send_registry_data: is_first_config,
            spawning_world: None,
            connection_ptr: None,
        }
    }

    pub fn player(&mut self) -> &mut Player {
        &mut self.player
    }

    pub fn into_player(self) -> Player {
        self.player
    }

    pub fn is_first_config(&self) -> bool {
        self.is_first_config
    }

    pub fn is_hardcore(&self) -> bool {
        self.is_hardcore
    }

    pub fn set_hardcore(&mut self, is_hardcore: bool) {
        self.is_hardcore = is_hardcore;
    }

    pub fn add_feature_flag(&mut self, feature_flag: Identifier) {
        if !self.feature_flags.contains(&feature_flag) {
            self.feature_flags.push(feature_flag);
        }
    }

    pub fn remove_feature_flag(&mut self, feature_flag: &Identifier) -> bool {
        let original_feature_flag_count = self.feature_flags.len();
        self.feature_flags
            .retain(|existing_feature_flag| existing_feature_flag != feature_flag);
        self.feature_flags.len() != original_feature_flag_count
    }

    pub fn feature_flags(&self) -> &[Identifier] {
        &self.feature_flags
    }

    pub fn should_clear_chat(&self) -> bool {
        self.should_clear_chat
    }

    pub fn set_should_clear_chat(&mut self, should_clear_chat: bool) {
        self.should_clear_chat = should_clear_chat;
    }

    pub fn should_send_registry_data(&self) -> bool {
        self.should_send_registry_data
    }

    pub fn set_should_send_registry_data(&mut self, should_send_registry_data: bool) {
        self.should_send_registry_data = should_send_registry_data;
    }

    pub fn spawning_world(&self) -> Option<Uuid> {
        self.spawning_world
    }

    pub fn set_spawning_world(&mut self, spawning_world: impl Into<Option<Uuid>>) {
        self.spawning_world = spawning_world.into();
    }
}
