use crate::entity::EntityPose;
use crate::entity::metadata::{MetadataHolder, definitions};
use crate::entity::player::{PlayerMeta, PlayerMetaRef};
use spinel_core::network::clientbound::play::entity_status::EntityStatusPacket;
use spinel_core::network::clientbound::play::set_entity_data::SetEntityDataPacket;
use spinel_network::types::MainHand;
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_network::types::{ClientInformation, Particle, Position};
use spinel_utils::component::text::TextComponent;
use std::io;

use super::hand::PlayerHand;
use super::state::Player;

fn client_information_main_hand(main_hand: i32) -> MainHand {
    match main_hand {
        0 => MainHand::Left,
        _ => MainHand::Right,
    }
}

impl Player {
    pub const fn has_reduced_debug_screen_information(&self) -> bool {
        self.reduced_debug_screen_information
    }

    pub fn set_reduced_debug_screen_information(
        &mut self,
        reduced_debug_screen_information: bool,
    ) -> io::Result<()> {
        self.reduced_debug_screen_information = reduced_debug_screen_information;
        let status = if reduced_debug_screen_information {
            22
        } else {
            23
        };
        self.send_packet(EntityStatusPacket {
            entity_id: self.get_entity_id().get_value(),
            status,
        })
    }

    pub const fn get_settings(&self) -> &ClientInformation {
        &self.settings
    }

    pub fn refresh_settings(&mut self, settings: ClientInformation) -> bool {
        let previous_view_distance = self.client_chunk_view_distance;
        self.client_chunk_view_distance = settings.view_distance.clamp(2, 32) as i32;
        self.settings = settings;
        self.settings.view_distance = self.settings.view_distance.clamp(2, 32);
        self.metadata.set(
            &definitions::avatar::displayed_model_parts_flags(),
            MetadataValue::Byte(self.settings.displayed_skin_parts as i8),
        );
        self.metadata.set(
            &definitions::avatar::get_main_hand(),
            MetadataValue::MainHand(client_information_main_hand(self.settings.main_hand)),
        );
        previous_view_distance != self.client_chunk_view_distance
    }

    pub fn get_locale(&self) -> &str {
        &self.settings.locale
    }

    pub fn set_locale(&mut self, locale: impl Into<String>) {
        let mut settings = self.settings.clone();
        settings.locale = locale.into();
        self.refresh_settings(settings);
    }

    pub fn get_player_metadata(&self) -> &MetadataHolder {
        &self.metadata
    }

    pub const fn get_entity_meta(&self) -> PlayerMetaRef<'_> {
        PlayerMetaRef::new(self)
    }

    pub fn get_entity_meta_mut(&mut self) -> PlayerMeta<'_> {
        PlayerMeta::new(self)
    }

    pub fn get_custom_name(&self) -> Option<TextComponent> {
        match self.metadata.get_value(&definitions::get_custom_name()) {
            MetadataValue::OptionalText(custom_name) => custom_name,
            _ => None,
        }
    }

    pub fn set_custom_name(&mut self, custom_name: Option<TextComponent>) {
        self.metadata.set(
            &definitions::get_custom_name(),
            MetadataValue::OptionalText(custom_name),
        );
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn is_custom_name_visible(&self) -> bool {
        match self.metadata.get_value(&definitions::custom_name_visible()) {
            MetadataValue::Boolean(custom_name_visible) => custom_name_visible,
            _ => false,
        }
    }

    pub fn set_custom_name_visible(&mut self, custom_name_visible: bool) {
        self.metadata.set(
            &definitions::custom_name_visible(),
            MetadataValue::Boolean(custom_name_visible),
        );
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn is_silent(&self) -> bool {
        match self.metadata.get_value(&definitions::is_silent()) {
            MetadataValue::Boolean(silent) => silent,
            _ => false,
        }
    }

    pub fn set_silent(&mut self, silent: bool) {
        self.metadata
            .set(&definitions::is_silent(), MetadataValue::Boolean(silent));
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn is_sneaking(&self) -> bool {
        self.metadata.get_flag(&definitions::is_crouching())
    }

    pub fn is_sprinting(&self) -> bool {
        self.metadata.get_flag(&definitions::is_sprinting())
    }

    pub fn is_swimming(&self) -> bool {
        self.metadata.get_flag(&definitions::is_swimming())
    }

    pub fn is_invisible(&self) -> bool {
        self.metadata.get_flag(&definitions::is_invisible())
    }

    pub fn is_glowing(&self) -> bool {
        self.metadata.get_flag(&definitions::has_glowing_effect())
    }

    pub fn is_flying_with_elytra(&self) -> bool {
        self.metadata
            .get_flag(&definitions::is_flying_with_elytra())
    }

    pub fn get_air_ticks(&self) -> i32 {
        match self.metadata.get_value(&definitions::get_air_ticks()) {
            MetadataValue::VarInt(air_ticks) => air_ticks,
            _ => 300,
        }
    }

    pub fn is_hand_active(&self) -> bool {
        self.metadata
            .get_flag(&definitions::living_entity::is_hand_active())
    }

    pub fn get_active_hand(&self) -> PlayerHand {
        if self
            .metadata
            .get_flag(&definitions::living_entity::get_active_hand())
        {
            return PlayerHand::Off;
        }
        PlayerHand::Main
    }

    pub fn is_in_riptide_spin_attack(&self) -> bool {
        self.metadata
            .get_flag(&definitions::living_entity::is_riptide_spin_attack())
    }

    pub fn get_effect_particles(&self) -> Vec<Particle> {
        match self
            .metadata
            .get_value(&definitions::living_entity::potion_effect_particles())
        {
            MetadataValue::ParticleList(effect_particles) => effect_particles,
            _ => Vec::new(),
        }
    }

    pub fn is_potion_effect_ambient(&self) -> bool {
        match self
            .metadata
            .get_value(&definitions::living_entity::is_potion_effect_ambient())
        {
            MetadataValue::Boolean(potion_effect_ambient) => potion_effect_ambient,
            _ => false,
        }
    }

    pub fn get_arrow_count(&self) -> i32 {
        match self
            .metadata
            .get_value(&definitions::living_entity::number_of_arrows())
        {
            MetadataValue::VarInt(arrow_count) => arrow_count,
            _ => 0,
        }
    }

    pub fn get_bee_stinger_count(&self) -> i32 {
        match self
            .metadata
            .get_value(&definitions::living_entity::number_of_bee_stingers())
        {
            MetadataValue::VarInt(bee_stinger_count) => bee_stinger_count,
            _ => 0,
        }
    }

    pub fn get_bed_in_which_sleeping_position(&self) -> Option<Position> {
        match self
            .metadata
            .get_value(&definitions::living_entity::location_of_bed())
        {
            MetadataValue::OptionalPosition(bed_position) => bed_position,
            _ => None,
        }
    }

    pub fn set_sneaking(&mut self, sneaking: bool) {
        self.metadata
            .set_flag(&definitions::is_crouching(), sneaking);
        if !self.is_flying() {
            self.refresh_pose();
        }
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn set_on_fire(&mut self, on_fire: bool) {
        self.metadata.set_flag(&definitions::is_on_fire(), on_fire);
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn get_pose(&self) -> EntityPose {
        match self.metadata.get_value(&definitions::get_pose()) {
            MetadataValue::Pose(pose) => {
                EntityPose::from_protocol_id(pose).unwrap_or(EntityPose::Standing)
            }
            _ => EntityPose::Standing,
        }
    }

    pub fn set_pose(&mut self, pose: EntityPose) {
        self.metadata.set(
            &definitions::get_pose(),
            MetadataValue::Pose(pose.get_protocol_id()),
        );
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn set_sprinting(&mut self, sprinting: bool) -> bool {
        let old_sprint = self.is_sprinting();
        self.metadata
            .set_flag(&definitions::is_sprinting(), sprinting);
        self.refresh_dirty_metadata_to_viewers();
        old_sprint != sprinting
    }

    pub fn set_swimming(&mut self, swimming: bool) {
        self.metadata
            .set_flag(&definitions::is_swimming(), swimming);
        self.refresh_pose();
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn set_invisible(&mut self, invisible: bool) {
        self.metadata
            .set_flag(&definitions::is_invisible(), invisible);
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn set_glowing(&mut self, glowing: bool) {
        self.metadata
            .set_flag(&definitions::has_glowing_effect(), glowing);
        self.refresh_dirty_metadata_to_viewers();
    }

    pub(crate) fn set_flying_with_elytra(&mut self, flying_with_elytra: bool) -> bool {
        let old_flying_with_elytra = self
            .metadata
            .get_flag(&definitions::is_flying_with_elytra());
        self.metadata
            .set_flag(&definitions::is_flying_with_elytra(), flying_with_elytra);
        self.refresh_pose();
        self.refresh_dirty_metadata_to_viewers();
        old_flying_with_elytra != flying_with_elytra
    }

    pub fn set_air_ticks(&mut self, air_ticks: i32) {
        self.metadata.set(
            &definitions::get_air_ticks(),
            MetadataValue::VarInt(air_ticks),
        );
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn set_hand_active(&mut self, hand_active: bool) {
        self.metadata
            .set_flag(&definitions::living_entity::is_hand_active(), hand_active);
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn set_active_hand(&mut self, hand: PlayerHand) {
        self.metadata.set_flag(
            &definitions::living_entity::get_active_hand(),
            hand == PlayerHand::Off,
        );
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn set_in_riptide_spin_attack(&mut self, in_riptide_spin_attack: bool) {
        self.metadata.set_flag(
            &definitions::living_entity::is_riptide_spin_attack(),
            in_riptide_spin_attack,
        );
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn set_effect_particles(&mut self, effect_particles: Vec<Particle>) {
        self.metadata.set(
            &definitions::living_entity::potion_effect_particles(),
            MetadataValue::ParticleList(effect_particles),
        );
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn set_potion_effect_ambient(&mut self, potion_effect_ambient: bool) {
        self.metadata.set(
            &definitions::living_entity::is_potion_effect_ambient(),
            MetadataValue::Boolean(potion_effect_ambient),
        );
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn set_arrow_count(&mut self, arrow_count: i32) {
        let normalized_arrow_count = arrow_count.max(0);
        self.living.set_arrow_count(normalized_arrow_count);
        self.metadata.set(
            &definitions::living_entity::number_of_arrows(),
            MetadataValue::VarInt(normalized_arrow_count),
        );
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn set_bee_stinger_count(&mut self, bee_stinger_count: i32) {
        self.metadata.set(
            &definitions::living_entity::number_of_bee_stingers(),
            MetadataValue::VarInt(bee_stinger_count.max(0)),
        );
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn set_bed_in_which_sleeping_position(&mut self, bed_position: Option<Position>) {
        self.metadata.set(
            &definitions::living_entity::location_of_bed(),
            MetadataValue::OptionalPosition(bed_position),
        );
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn get_main_hand(&self) -> MainHand {
        match self
            .metadata
            .get_value(&definitions::avatar::get_main_hand())
        {
            MetadataValue::MainHand(main_hand) => main_hand,
            _ => MainHand::Right,
        }
    }

    pub fn set_main_hand(&mut self, main_hand: MainHand) {
        self.metadata.set(
            &definitions::avatar::get_main_hand(),
            MetadataValue::MainHand(main_hand),
        );
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn is_cape_enabled(&self) -> bool {
        self.metadata
            .get_flag(&definitions::avatar::is_cape_enabled())
    }

    pub fn set_cape_enabled(&mut self, cape_enabled: bool) {
        self.metadata
            .set_flag(&definitions::avatar::is_cape_enabled(), cape_enabled);
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn is_jacket_enabled(&self) -> bool {
        self.metadata
            .get_flag(&definitions::avatar::is_jacket_enabled())
    }

    pub fn set_jacket_enabled(&mut self, jacket_enabled: bool) {
        self.metadata
            .set_flag(&definitions::avatar::is_jacket_enabled(), jacket_enabled);
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn is_left_sleeve_enabled(&self) -> bool {
        self.metadata
            .get_flag(&definitions::avatar::is_left_sleeve_enabled())
    }

    pub fn set_left_sleeve_enabled(&mut self, left_sleeve_enabled: bool) {
        self.metadata.set_flag(
            &definitions::avatar::is_left_sleeve_enabled(),
            left_sleeve_enabled,
        );
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn is_right_sleeve_enabled(&self) -> bool {
        self.metadata
            .get_flag(&definitions::avatar::is_right_sleeve_enabled())
    }

    pub fn set_right_sleeve_enabled(&mut self, right_sleeve_enabled: bool) {
        self.metadata.set_flag(
            &definitions::avatar::is_right_sleeve_enabled(),
            right_sleeve_enabled,
        );
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn is_left_leg_enabled(&self) -> bool {
        self.metadata
            .get_flag(&definitions::avatar::is_left_pants_leg_enabled())
    }

    pub fn set_left_leg_enabled(&mut self, left_leg_enabled: bool) {
        self.metadata.set_flag(
            &definitions::avatar::is_left_pants_leg_enabled(),
            left_leg_enabled,
        );
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn is_right_leg_enabled(&self) -> bool {
        self.metadata
            .get_flag(&definitions::avatar::is_right_pants_leg_enabled())
    }

    pub fn set_right_leg_enabled(&mut self, right_leg_enabled: bool) {
        self.metadata.set_flag(
            &definitions::avatar::is_right_pants_leg_enabled(),
            right_leg_enabled,
        );
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn is_hat_enabled(&self) -> bool {
        self.metadata
            .get_flag(&definitions::avatar::is_hat_enabled())
    }

    pub fn set_hat_enabled(&mut self, hat_enabled: bool) {
        self.metadata
            .set_flag(&definitions::avatar::is_hat_enabled(), hat_enabled);
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn get_displayed_skin_parts(&self) -> i8 {
        match self
            .metadata
            .get_value(&definitions::avatar::displayed_model_parts_flags())
        {
            MetadataValue::Byte(displayed_skin_parts) => displayed_skin_parts,
            _ => 0,
        }
    }

    pub fn set_displayed_skin_parts(&mut self, displayed_skin_parts: i8) {
        self.metadata.set(
            &definitions::avatar::displayed_model_parts_flags(),
            MetadataValue::Byte(displayed_skin_parts),
        );
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn get_score(&self) -> i32 {
        match self.metadata.get_value(&definitions::player::get_score()) {
            MetadataValue::VarInt(score) => score,
            _ => 0,
        }
    }

    pub fn set_score(&mut self, score: i32) {
        self.metadata.set(
            &definitions::player::get_score(),
            MetadataValue::VarInt(score),
        );
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn get_left_shoulder_entity_data(&self) -> Option<i32> {
        match self
            .metadata
            .get_value(&definitions::player::get_left_shoulder_entity_data())
        {
            MetadataValue::OptionalVarInt(left_shoulder_entity_data) => left_shoulder_entity_data,
            _ => None,
        }
    }

    pub fn set_left_shoulder_entity_data(&mut self, left_shoulder_entity_data: Option<i32>) {
        self.metadata.set(
            &definitions::player::get_left_shoulder_entity_data(),
            MetadataValue::OptionalVarInt(left_shoulder_entity_data),
        );
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn get_right_shoulder_entity_data(&self) -> Option<i32> {
        match self
            .metadata
            .get_value(&definitions::player::get_right_shoulder_entity_data())
        {
            MetadataValue::OptionalVarInt(right_shoulder_entity_data) => right_shoulder_entity_data,
            _ => None,
        }
    }

    pub fn set_right_shoulder_entity_data(&mut self, right_shoulder_entity_data: Option<i32>) {
        self.metadata.set(
            &definitions::player::get_right_shoulder_entity_data(),
            MetadataValue::OptionalVarInt(right_shoulder_entity_data),
        );
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn leave_bed(&mut self) {
        self.metadata
            .set(&definitions::get_pose(), MetadataValue::Pose(0));
    }

    pub(crate) fn get_metadata_packet(&self) -> SetEntityDataPacket {
        SetEntityDataPacket::new(
            self.get_entity_id().get_value(),
            self.metadata.get_entries(),
        )
    }

    pub(crate) fn get_dirty_metadata_packet(&mut self) -> Option<SetEntityDataPacket> {
        let dirty_entries = self.metadata.drain_dirty_entries();
        if dirty_entries.is_empty() {
            return None;
        }
        Some(SetEntityDataPacket::new(
            self.get_entity_id().get_value(),
            dirty_entries,
        ))
    }

    pub(in crate::entity::player) fn refresh_pose(&mut self) {
        let pose = if self
            .metadata
            .get_flag(&definitions::is_flying_with_elytra())
        {
            EntityPose::FallFlying
        } else if self.metadata.get_flag(&definitions::is_swimming()) {
            EntityPose::Swimming
        } else if self.metadata.get_flag(&definitions::is_crouching()) {
            EntityPose::Sneaking
        } else {
            EntityPose::Standing
        };
        self.metadata.set(
            &definitions::get_pose(),
            MetadataValue::Pose(pose.get_protocol_id()),
        );
    }

    pub(super) fn refresh_dirty_metadata_to_viewers(&mut self) {
        if !self.has_entered_world() {
            return;
        }
        let Some(metadata_packet) = self.get_dirty_metadata_packet() else {
            return;
        };
        let metadata_entity_id = metadata_packet.entity_id;
        let metadata_entries = metadata_packet.entries.0;
        let _ = self.dispatch_to_viewer_clients(|viewer_client| {
            SetEntityDataPacket::new(metadata_entity_id, metadata_entries.clone())
                .dispatch(viewer_client)
        });
    }
}
