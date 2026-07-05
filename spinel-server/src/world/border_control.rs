impl World {
    pub const fn get_world_border(&self) -> WorldBorder {
        self.world_border
    }

    pub fn block_position_is_inside_world_border(&self, position: BlockPosition) -> bool {
        self.world_border
            .contains(f64::from(position.x) + 0.5, f64::from(position.z) + 0.5)
    }

    pub fn set_world_border(&mut self, world_border: WorldBorder) {
        let _ = self.set_world_border_with_transition(world_border, 0);
    }

    pub fn set_world_border_with_transition(
        &mut self,
        world_border: WorldBorder,
        transition_time: i64,
    ) -> Result<()> {
        self.world_border = world_border;
        let packet = self.create_initialize_world_border_packet_with_transition(transition_time);
        self.dispatch_packet_to_entered_players(packet)
    }

    pub fn create_initialize_world_border_packet(
        &self,
    ) -> spinel_core::network::clientbound::play::initialize_world_border::InitializeWorldBorderPacket
    {
        self.create_initialize_world_border_packet_with_transition(0)
    }

    fn create_initialize_world_border_packet_with_transition(
        &self,
        transition_time: i64,
    ) -> spinel_core::network::clientbound::play::initialize_world_border::InitializeWorldBorderPacket
    {
        self.world_border
            .initialize_packet(self.world_border.diameter(), transition_time)
    }
}
