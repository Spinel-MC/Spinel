impl World {
    pub fn show_boss_bar(&mut self, boss_bar: BossBar) -> Result<bool> {
        if self
            .boss_bars
            .iter()
            .any(|stored_bar| stored_bar.id() == boss_bar.id())
        {
            return Ok(false);
        }
        let packet = boss_bar.add_packet();
        self.boss_bars.push(boss_bar);
        self.dispatch_packet_to_entered_players(packet)?;
        Ok(true)
    }

    pub fn hide_boss_bar(&mut self, boss_bar_id: Uuid) -> Result<bool> {
        let Some(boss_bar_index) = self
            .boss_bars
            .iter()
            .position(|boss_bar| boss_bar.id() == boss_bar_id)
        else {
            return Ok(false);
        };
        let boss_bar = self.boss_bars.remove(boss_bar_index);
        self.dispatch_packet_to_entered_players(boss_bar.remove_packet())?;
        Ok(true)
    }

    pub fn boss_bars(&self) -> &[BossBar] {
        &self.boss_bars
    }
}
