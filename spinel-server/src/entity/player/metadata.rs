use crate::entity::player::Player;
use spinel_network::types::MainHand;

pub struct PlayerMeta<'player> {
    player: &'player mut Player,
}

impl<'player> PlayerMeta<'player> {
    pub(crate) fn new(player: &'player mut Player) -> Self {
        Self { player }
    }

    pub fn get_main_hand(&self) -> MainHand {
        self.player.get_main_hand()
    }

    pub fn set_main_hand(&mut self, main_hand: MainHand) {
        self.player.set_main_hand(main_hand);
    }

    pub fn is_cape_enabled(&self) -> bool {
        self.player.is_cape_enabled()
    }

    pub fn set_cape_enabled(&mut self, is_cape_enabled: bool) {
        self.player.set_cape_enabled(is_cape_enabled);
    }

    pub fn is_jacket_enabled(&self) -> bool {
        self.player.is_jacket_enabled()
    }

    pub fn set_jacket_enabled(&mut self, is_jacket_enabled: bool) {
        self.player.set_jacket_enabled(is_jacket_enabled);
    }

    pub fn is_left_sleeve_enabled(&self) -> bool {
        self.player.is_left_sleeve_enabled()
    }

    pub fn set_left_sleeve_enabled(&mut self, is_left_sleeve_enabled: bool) {
        self.player.set_left_sleeve_enabled(is_left_sleeve_enabled);
    }

    pub fn is_right_sleeve_enabled(&self) -> bool {
        self.player.is_right_sleeve_enabled()
    }

    pub fn set_right_sleeve_enabled(&mut self, is_right_sleeve_enabled: bool) {
        self.player
            .set_right_sleeve_enabled(is_right_sleeve_enabled);
    }

    pub fn is_left_leg_enabled(&self) -> bool {
        self.player.is_left_leg_enabled()
    }

    pub fn set_left_leg_enabled(&mut self, is_left_leg_enabled: bool) {
        self.player.set_left_leg_enabled(is_left_leg_enabled);
    }

    pub fn is_right_leg_enabled(&self) -> bool {
        self.player.is_right_leg_enabled()
    }

    pub fn set_right_leg_enabled(&mut self, is_right_leg_enabled: bool) {
        self.player.set_right_leg_enabled(is_right_leg_enabled);
    }

    pub fn is_hat_enabled(&self) -> bool {
        self.player.is_hat_enabled()
    }

    pub fn set_hat_enabled(&mut self, is_hat_enabled: bool) {
        self.player.set_hat_enabled(is_hat_enabled);
    }

    pub fn get_displayed_skin_parts(&self) -> i8 {
        self.player.get_displayed_skin_parts()
    }

    pub fn set_displayed_skin_parts(&mut self, displayed_skin_parts: i8) {
        self.player.set_displayed_skin_parts(displayed_skin_parts);
    }

    pub fn get_additional_hearts(&self) -> f32 {
        self.player.get_additional_hearts()
    }

    pub fn set_additional_hearts(&mut self, additional_hearts: f32) {
        self.player.set_additional_hearts(additional_hearts);
    }

    pub fn get_score(&self) -> i32 {
        self.player.get_score()
    }

    pub fn set_score(&mut self, score: i32) {
        self.player.set_score(score);
    }

    pub fn get_left_shoulder_entity_data(&self) -> Option<i32> {
        self.player.get_left_shoulder_entity_data()
    }

    pub fn set_left_shoulder_entity_data(&mut self, left_shoulder_entity_data: Option<i32>) {
        self.player
            .set_left_shoulder_entity_data(left_shoulder_entity_data);
    }

    pub fn get_right_shoulder_entity_data(&self) -> Option<i32> {
        self.player.get_right_shoulder_entity_data()
    }

    pub fn set_right_shoulder_entity_data(&mut self, right_shoulder_entity_data: Option<i32>) {
        self.player
            .set_right_shoulder_entity_data(right_shoulder_entity_data);
    }
}
