use crate::entity::EntityId;
use crate::entity::player::BelowNameTag;
use crate::scoreboard::Team;
use crate::world::BossBar;
use spinel_core::network::clientbound::play::advancements::Notification;
use spinel_core::network::clientbound::play::clear_dialog::ClearDialogPacket;
use spinel_core::network::clientbound::play::clear_titles::ClearTitlesPacket;
use spinel_core::network::clientbound::play::entity_sound_effect::{
    EntitySoundEffectPacket, NetworkSoundEvent,
};
use spinel_core::network::clientbound::play::game_event::{GameEvent, GameEventPacket};
use spinel_core::network::clientbound::play::open_book::OpenBookPacket;
use spinel_core::network::clientbound::play::plugin_message::PlayCustomPayloadPacket;
use spinel_core::network::clientbound::play::recipe_book_add::RecipeBookAddPacket;
use spinel_core::network::clientbound::play::set_camera::SetCameraPacket;
use spinel_core::network::clientbound::play::set_subtitle_text::SetSubtitleTextPacket;
use spinel_core::network::clientbound::play::set_title_text::SetTitleTextPacket;
use spinel_core::network::clientbound::play::set_titles_animation::SetTitlesAnimationPacket;
use spinel_core::network::clientbound::play::show_dialog::ShowDialogPacket;
use spinel_core::network::clientbound::play::sound_effect::{
    NetworkPositionedSoundEvent, SoundEffectPacket,
};
use spinel_core::network::clientbound::play::stop_sound::StopSoundPacket;
use spinel_core::network::clientbound::play::system_chat::SystemChatPacket;
use spinel_core::network::clientbound::play::tab_list::TabListPacket;
use spinel_core::network::clientbound::play::update_recipes::UpdateRecipesPacket;
use spinel_core::network::clientbound::play::world_event::WorldEventPacket;
use spinel_network::types::sound::SoundEvent;
use spinel_network::types::{Identifier, Position, Vector3d};
use spinel_network::{ConnectionState, PacketSender, PacketStruct};
use spinel_registry::RegistryKey;
use spinel_registry::dialog::Dialog;
use spinel_utils::component::text::TextComponent;
use std::io;
use uuid::Uuid;

use super::hand::PlayerHand;
use super::message_type::PlayerMessageType;
use super::state::Player;

impl Player {
    pub fn refresh_recipes(&mut self) -> io::Result<()> {
        let Some(client) = self.get_client_mut() else {
            return Ok(());
        };
        UpdateRecipesPacket.dispatch(client)?;
        RecipeBookAddPacket::reset_empty().dispatch(client)
    }

    pub fn send_packet<P>(&mut self, packet: P) -> io::Result<()>
    where
        P: PacketStruct,
    {
        let Some(client) = self.get_client_mut() else {
            return Ok(());
        };
        if client.state != P::get_state() {
            return Ok(());
        }

        let mut payload = Vec::new();
        packet.encode(&mut payload)?;
        client.send_packet(P::get_id(), &payload)
    }

    pub fn send_packets<P>(&mut self, packets: impl IntoIterator<Item = P>) -> io::Result<()>
    where
        P: PacketStruct,
    {
        packets
            .into_iter()
            .try_for_each(|packet| self.send_packet(packet))
    }

    pub fn send_raw_packet(&mut self, packet_id: i32, payload: &[u8]) -> io::Result<()> {
        let Some(client) = self.get_client_mut() else {
            return Ok(());
        };

        client.send_packet(packet_id, payload)
    }

    pub fn send_plugin_message(
        &mut self,
        channel: impl Into<String>,
        data: Vec<u8>,
    ) -> io::Result<()> {
        self.send_packet(PlayCustomPayloadPacket::new(channel, data))
    }

    pub fn send_plugin_message_string(
        &mut self,
        channel: impl Into<String>,
        data: impl Into<String>,
    ) -> io::Result<()> {
        self.send_plugin_message(channel, data.into().into_bytes())
    }

    pub fn send_message(&mut self, message: impl Into<TextComponent>) {
        let _ = self.send_packet(SystemChatPacket::new(message.into(), false));
    }

    pub fn send_system_message(&mut self, message: impl Into<TextComponent>) {
        self.send_message(message);
    }

    pub fn send_action_bar(&mut self, message: impl Into<TextComponent>) {
        let _ = self.send_packet(SystemChatPacket::new(message.into(), true));
    }

    pub fn send_message_from(
        &mut self,
        _source: Uuid,
        message: impl Into<TextComponent>,
        message_type: PlayerMessageType,
    ) -> io::Result<()> {
        if !message_type.is_accepted_by_chat_mode(self.settings.chat_mode) {
            return Ok(());
        }
        self.send_packet(SystemChatPacket::new(
            message,
            message_type == PlayerMessageType::ActionBar,
        ))
    }

    pub fn can_receive_chat_message(&self) -> bool {
        self.settings.chat_mode == 0
    }

    pub fn can_receive_chat_command(&self) -> bool {
        self.settings.chat_mode != 2
    }

    pub fn send_chat_rejection_message(&mut self) -> io::Result<()> {
        self.send_packet(SystemChatPacket::new(
            TextComponent::translatable("chat.cannotSend")
                .color(spinel_utils::component::color::TextColor::from_named(
                    spinel_utils::component::color::NamedTextColor::Red,
                ))
                .build(),
            false,
        ))
    }

    pub fn open_book(&mut self, hand: PlayerHand) -> io::Result<()> {
        self.send_packet(OpenBookPacket::new(hand.get_protocol_id()))
    }

    pub fn show_dialog(&mut self, dialog: &RegistryKey<Dialog>) -> io::Result<()> {
        let packet = ShowDialogPacket::from_vanilla_dialog(dialog).ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "unknown registered dialog")
        })?;
        self.send_packet(packet)
    }

    pub fn send_player_list_header_and_footer(
        &mut self,
        header: TextComponent,
        footer: TextComponent,
    ) -> io::Result<()> {
        self.send_packet(TabListPacket::new(header, footer))
    }

    pub fn send_title(&mut self, title: TextComponent) -> io::Result<()> {
        self.send_packet(SetTitleTextPacket::new(title))
    }

    pub fn send_subtitle(&mut self, subtitle: TextComponent) -> io::Result<()> {
        self.send_packet(SetSubtitleTextPacket::new(subtitle))
    }

    pub fn set_title_animation(
        &mut self,
        fade_in: i32,
        stay: i32,
        fade_out: i32,
    ) -> io::Result<()> {
        self.send_packet(SetTitlesAnimationPacket::new(fade_in, stay, fade_out))
    }

    pub fn clear_title(&mut self) -> io::Result<()> {
        self.send_packet(ClearTitlesPacket::clear())
    }

    pub fn close_dialog(&mut self) -> io::Result<()> {
        self.send_packet(ClearDialogPacket)
    }

    pub fn send_notification(&mut self, notification: Notification) -> io::Result<()> {
        let timestamp_millis = self.alive_ticks as i64 * 50;
        self.send_packet(notification.add_packet(timestamp_millis))?;
        self.send_packet(notification.remove_packet())
    }

    pub fn get_below_name_tag(&self) -> Option<&BelowNameTag> {
        self.below_name_tag.as_ref()
    }

    pub fn set_below_name_tag(&mut self, below_name_tag: Option<BelowNameTag>) -> io::Result<()> {
        if self.below_name_tag == below_name_tag {
            return Ok(());
        }

        if let Some(previous_below_name_tag) = self.below_name_tag.take() {
            self.send_packet(previous_below_name_tag.remove_packet())?;
        }

        self.below_name_tag = below_name_tag;
        if let Some(current_below_name_tag) = self.below_name_tag.clone() {
            self.send_packet(current_below_name_tag.create_packet())?;
            self.send_packet(current_below_name_tag.get_display_packet())?;
        }

        Ok(())
    }

    pub fn get_team(&self) -> Option<&str> {
        self.living.get_team()
    }

    pub fn set_scoreboard_team(
        &mut self,
        mut previous_team: Option<&mut Team>,
        mut new_team: Option<&mut Team>,
    ) -> Vec<spinel_core::network::clientbound::play::set_player_team::SetPlayerTeamPacket> {
        let member = self.username.clone();
        let mut packets = Vec::new();
        if let Some(previous_team_name) = self.living.get_team().map(str::to_owned) {
            self.living.set_team(None);
            let should_remove_previous_member = new_team
                .as_ref()
                .is_none_or(|current_team| current_team.name() != previous_team_name);
            if should_remove_previous_member {
                if let Some(previous_team) = previous_team.as_mut() {
                    if let Some(packet) = previous_team.remove_member(&member) {
                        packets.push(packet);
                    }
                } else {
                    packets.push(
                        spinel_core::network::clientbound::play::set_player_team::SetPlayerTeamPacket {
                            team_name: previous_team_name,
                            action: spinel_core::network::clientbound::play::set_player_team::TeamAction::RemoveEntities(vec![member.clone()]),
                        },
                    );
                }
            }
        }
        if let Some(current_team) = new_team.as_mut() {
            self.living.set_team(Some(current_team.name().to_owned()));
            if let Some(packet) = current_team.add_member(member) {
                packets.push(packet);
            }
        }
        packets
    }

    pub fn reset_title(&mut self) -> io::Result<()> {
        self.send_packet(ClearTitlesPacket::reset())
    }

    pub fn play_sound(&mut self, sound_event: SoundEvent) -> io::Result<()> {
        self.play_sound_at_position(sound_event, self.get_position().as_vector())
    }

    pub fn play_sound_at_position(
        &mut self,
        sound_event: SoundEvent,
        position: Vector3d,
    ) -> io::Result<()> {
        self.send_packet(SoundEffectPacket {
            sound_event: NetworkPositionedSoundEvent(sound_event),
            source_id: 0,
            position,
            volume: 1.0,
            pitch: 1.0,
            seed: 0,
        })
    }

    pub fn play_sound_from_entity(
        &mut self,
        sound_event: SoundEvent,
        entity_id: EntityId,
    ) -> io::Result<()> {
        self.send_packet(EntitySoundEffectPacket {
            sound_event: NetworkSoundEvent(sound_event),
            source_id: 0,
            entity_id: entity_id.get_value(),
            volume: 1.0,
            pitch: 1.0,
            seed: 0,
        })
    }

    pub fn stop_sound(&mut self, source: Option<i32>, sound: Option<Identifier>) -> io::Result<()> {
        self.send_packet(StopSoundPacket::new(source, sound))
    }

    pub fn play_effect(
        &mut self,
        effect_id: i32,
        position: Position,
        data: i32,
        global_event: bool,
    ) -> io::Result<()> {
        self.send_packet(WorldEventPacket::new(
            effect_id,
            position,
            data,
            global_event,
        ))
    }

    pub fn show_boss_bar(&mut self, boss_bar: &BossBar) -> io::Result<()> {
        self.send_packet(boss_bar.add_packet())
    }

    pub fn hide_boss_bar(&mut self, boss_bar: &BossBar) -> io::Result<()> {
        self.send_packet(boss_bar.remove_packet())
    }

    pub fn spectate(&mut self, entity_id: EntityId) -> io::Result<()> {
        self.send_packet(SetCameraPacket::new(entity_id.get_value()))
    }

    pub fn stop_spectating(&mut self) -> io::Result<()> {
        self.send_packet(SetCameraPacket::new(self.get_entity_id().get_value()))
    }

    pub(super) fn dispatch_game_event(&mut self, game_event: GameEvent) -> io::Result<()> {
        let packet = GameEventPacket::from(game_event);
        let Some(client) = self.get_client_mut() else {
            return Ok(());
        };
        if client.state != ConnectionState::Play {
            return Ok(());
        }
        packet.dispatch(client)
    }
}
