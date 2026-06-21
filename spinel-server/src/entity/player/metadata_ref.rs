use crate::entity::{EntityPose, Player, PlayerHand};
use spinel_network::types::{Particle, Position};
use spinel_utils::component::text::TextComponent;

#[derive(Clone, Copy)]
pub struct PlayerMetaRef<'player> {
    player: &'player Player,
}

impl<'player> PlayerMetaRef<'player> {
    pub(crate) const fn new(player: &'player Player) -> Self {
        Self { player }
    }

    pub fn is_hand_active(&self) -> bool {
        self.player.is_hand_active()
    }

    pub fn get_active_hand(&self) -> PlayerHand {
        self.player.get_active_hand()
    }

    pub fn get_effect_particles(&self) -> Vec<Particle> {
        self.player.get_effect_particles()
    }

    pub fn is_potion_effect_ambient(&self) -> bool {
        self.player.is_potion_effect_ambient()
    }

    pub fn get_arrow_count(&self) -> i32 {
        self.player.get_arrow_count()
    }

    pub fn get_bee_stinger_count(&self) -> i32 {
        self.player.get_bee_stinger_count()
    }

    pub fn get_bed_in_which_sleeping_position(&self) -> Option<Position> {
        self.player.get_bed_in_which_sleeping_position()
    }

    pub fn get_custom_name(&self) -> Option<TextComponent> {
        self.player.get_custom_name()
    }

    pub fn is_custom_name_visible(&self) -> bool {
        self.player.is_custom_name_visible()
    }

    pub fn is_silent(&self) -> bool {
        self.player.is_silent()
    }

    pub fn has_no_gravity(&self) -> bool {
        self.player.has_no_gravity()
    }

    pub fn get_pose(&self) -> EntityPose {
        self.player.get_pose()
    }

    pub fn ticks_frozen(&self) -> i32 {
        self.player.ticks_frozen()
    }
}
