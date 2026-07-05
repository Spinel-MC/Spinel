impl World {
    pub fn send_block_action(
        &mut self,
        position: BlockPosition,
        action_id: u8,
        action_param: u8,
    ) -> Result<()> {
        let chunk_position =
            ChunkPosition::new(position.x.div_euclid(16), position.z.div_euclid(16));
        let Some(chunk) = self
            .chunks
            .get(&chunk_position)
            .filter(|chunk| chunk.is_loaded())
        else {
            return Err(Error::new(ErrorKind::NotFound, "Chunk is not loaded"));
        };
        let block = chunk.block(position);
        let packet = BlockActionPacket::new(
            Position {
                x: position.x,
                y: position.y,
                z: position.z,
            },
            action_id,
            action_param,
            block.state_id(),
        );
        self.dispatch_packet_to_chunk_viewers(chunk_position, packet)
    }

    pub fn play_sound_except(
        &mut self,
        excluded_player: Option<Uuid>,
        sound_event: SoundEvent,
        source_id: i32,
        position: EntityPosition,
        volume: f32,
        pitch: f32,
        seed: i64,
    ) -> Result<()> {
        self.entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Player(player) if player.has_entered_world() => Some(player),
                _ => None,
            })
            .filter(|player| Some(player.get_uuid()) != excluded_player)
            .filter_map(Player::get_client_mut)
            .try_for_each(|client| {
                SoundEffectPacket {
                    sound_event: NetworkPositionedSoundEvent(sound_event.clone()),
                    source_id,
                    position: Vector3d {
                        x: position.get_x(),
                        y: position.get_y(),
                        z: position.get_z(),
                    },
                    volume,
                    pitch,
                    seed,
                }
                .dispatch(client)
            })
    }

    pub fn play_sound_except_emitter(
        &mut self,
        excluded_player: Option<Uuid>,
        sound_event: SoundEvent,
        source_id: i32,
        emitter: WorldSoundEmitter,
        volume: f32,
        pitch: f32,
        seed: i64,
    ) -> Result<()> {
        match emitter {
            WorldSoundEmitter::Entity(entity_id) => self.play_entity_sound_except(
                excluded_player,
                sound_event,
                source_id,
                entity_id,
                volume,
                pitch,
                seed,
            ),
            WorldSoundEmitter::SelfPlayer => self.play_self_emitter_sound_except(
                excluded_player,
                sound_event,
                source_id,
                volume,
                pitch,
                seed,
            ),
        }
    }

    fn play_entity_sound_except(
        &mut self,
        excluded_player: Option<Uuid>,
        sound_event: SoundEvent,
        source_id: i32,
        entity_id: EntityId,
        volume: f32,
        pitch: f32,
        seed: i64,
    ) -> Result<()> {
        self.entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Player(player) if player.has_entered_world() => Some(player),
                _ => None,
            })
            .filter(|player| Some(player.get_uuid()) != excluded_player)
            .filter_map(Player::get_client_mut)
            .try_for_each(|client| {
                EntitySoundEffectPacket {
                    sound_event: NetworkSoundEvent(sound_event.clone()),
                    source_id,
                    entity_id: entity_id.get_value() as i32,
                    volume,
                    pitch,
                    seed,
                }
                .dispatch(client)
            })
    }

    fn play_self_emitter_sound_except(
        &mut self,
        excluded_player: Option<Uuid>,
        sound_event: SoundEvent,
        source_id: i32,
        volume: f32,
        pitch: f32,
        seed: i64,
    ) -> Result<()> {
        self.entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Player(player) if player.has_entered_world() => Some(player),
                _ => None,
            })
            .filter(|player| Some(player.get_uuid()) != excluded_player)
            .try_for_each(|player| {
                let entity_id = player.get_entity_id().get_value() as i32;
                let Some(client) = player.get_client_mut() else {
                    return Ok(());
                };
                EntitySoundEffectPacket {
                    sound_event: NetworkSoundEvent(sound_event.clone()),
                    source_id,
                    entity_id,
                    volume,
                    pitch,
                    seed,
                }
                .dispatch(client)
            })
    }

    fn dispatch_packet_to_chunk_viewers<P>(
        &mut self,
        position: ChunkPosition,
        packet: P,
    ) -> Result<()>
    where
        P: DataType + PacketStruct,
    {
        let Some(chunk) = self.chunks.get(&position) else {
            return Ok(());
        };
        let viewer_ids = chunk.viewers().collect::<HashSet<_>>();
        let mut payload = Vec::new();
        packet.encode(&mut payload)?;
        self.entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Player(player)
                    if viewer_ids.contains(&player.get_entity_id().get_value()) =>
                {
                    Some(player)
                }
                _ => None,
            })
            .try_for_each(|player| {
                let Some(client) = player.get_client_mut() else {
                    return Ok(());
                };
                client.send_packet(P::get_id(), &payload)
            })
    }

    fn dispatch_packet_to_entered_players<P>(&mut self, packet: P) -> Result<()>
    where
        P: DataType + PacketStruct,
    {
        self.dispatch_packet_to_players(packet)
    }

    pub(crate) fn dispatch_packet_to_players<P>(&mut self, packet: P) -> Result<()>
    where
        P: DataType + PacketStruct,
    {
        let mut payload = Vec::new();
        packet.encode(&mut payload)?;
        self.entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Player(player) if player.has_entered_world() => Some(player),
                _ => None,
            })
            .filter_map(Player::get_client_mut)
            .try_for_each(|client| client.send_packet(P::get_id(), &payload))
    }
}
