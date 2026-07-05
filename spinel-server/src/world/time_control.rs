impl World {
    pub const fn world_age(&self) -> i64 {
        self.world_age
    }

    pub fn set_world_age(&mut self, world_age: i64) {
        self.world_age = world_age;
        let _ = self.broadcast_time();
    }

    pub const fn time(&self) -> i64 {
        self.time
    }

    pub fn set_time(&mut self, time: i64) {
        self.time = time;
        let _ = self.broadcast_time();
    }

    pub const fn time_rate(&self) -> i32 {
        self.time_rate
    }

    pub fn set_time_rate(&mut self, time_rate: i32) -> Result<()> {
        if time_rate < 0 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Time rate cannot be lower than 0",
            ));
        }
        self.time_rate = time_rate;
        Ok(())
    }

    pub const fn time_synchronization_ticks(&self) -> i32 {
        self.time_synchronization_ticks
    }

    pub fn set_time_synchronization_ticks(
        &mut self,
        time_synchronization_ticks: i32,
    ) -> Result<()> {
        if time_synchronization_ticks < 0 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Time synchronization ticks cannot be lower than 0",
            ));
        }
        self.time_synchronization_ticks = time_synchronization_ticks;
        Ok(())
    }

    pub const fn weather(&self) -> Weather {
        self.weather
    }

    pub fn set_weather(&mut self, weather: Weather) {
        self.weather = weather;
        self.remaining_rain_transition_ticks = self.default_rain_transition_ticks(weather);
        self.remaining_thunder_transition_ticks = self.default_thunder_transition_ticks(weather);
    }

    pub fn set_weather_with_transition(
        &mut self,
        weather: Weather,
        transition_ticks: i32,
    ) -> Result<()> {
        if transition_ticks < 1 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Transition ticks cannot be lower than 1",
            ));
        }
        self.weather = weather;
        self.remaining_rain_transition_ticks = transition_ticks;
        self.remaining_thunder_transition_ticks = transition_ticks;
        Ok(())
    }

    pub const fn transitioning_weather(&self) -> Weather {
        self.transitioning_weather
    }

    pub const fn remaining_rain_transition_ticks(&self) -> i32 {
        self.remaining_rain_transition_ticks
    }

    pub const fn remaining_thunder_transition_ticks(&self) -> i32 {
        self.remaining_thunder_transition_ticks
    }

    fn default_rain_transition_ticks(&self, weather: Weather) -> i32 {
        ((weather.rain_level() - self.transitioning_weather.rain_level()).abs() / 0.01).max(1.0)
            as i32
    }

    fn default_thunder_transition_ticks(&self, weather: Weather) -> i32 {
        ((weather.thunder_level() - self.transitioning_weather.thunder_level()).abs() / 0.01)
            .max(1.0) as i32
    }

    pub const fn time_packet(&self) -> SetTimePacket {
        SetTimePacket::new(self.world_age, self.time, self.time_rate != 0)
    }

    fn tick_time(&mut self) {
        self.world_age += 1;
        self.time += self.time_rate as i64;
        if self.time_synchronization_ticks <= 0 {
            return;
        }
        if self.world_age % self.time_synchronization_ticks as i64 != 0 {
            return;
        }
        let _ = self.broadcast_time();
    }

    fn broadcast_time(&mut self) -> Result<()> {
        let time_packet = self.time_packet();
        self.entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Player(player) if player.has_entered_world() => Some(player),
                _ => None,
            })
            .filter_map(Player::get_client_mut)
            .try_for_each(|client| {
                SetTimePacket::new(
                    time_packet.world_age,
                    time_packet.time,
                    time_packet.tick_day_time,
                )
                .dispatch(client)
            })
    }

    fn broadcast_weather(&mut self, previous_weather: Weather) -> Result<()> {
        let weather = self.transitioning_weather;
        self.entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Player(player) if player.has_entered_world() => Some(player),
                _ => None,
            })
            .filter_map(Player::get_client_mut)
            .try_for_each(|client| {
                if previous_weather.is_raining() != weather.is_raining() {
                    weather.is_raining_packet().dispatch(client)?;
                }
                if previous_weather.rain_level() != weather.rain_level() {
                    weather.rain_level_packet().dispatch(client)?;
                }
                if previous_weather.thunder_level() == weather.thunder_level() {
                    return Ok(());
                }
                weather.thunder_level_packet().dispatch(client)
            })
    }

    fn tick_weather(&mut self) {
        if self.remaining_rain_transition_ticks <= 0 && self.remaining_thunder_transition_ticks <= 0
        {
            return;
        }
        let previous_weather = self.transitioning_weather;
        self.transitioning_weather = transition_weather(
            self.weather,
            self.transitioning_weather,
            self.remaining_rain_transition_ticks,
            self.remaining_thunder_transition_ticks,
        );
        let _ = self.broadcast_weather(previous_weather);
        self.remaining_rain_transition_ticks = (self.remaining_rain_transition_ticks - 1).max(0);
        self.remaining_thunder_transition_ticks =
            (self.remaining_thunder_transition_ticks - 1).max(0);
    }
}

fn current_time_nanos() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or_default()
}

fn transition_weather(
    target_weather: Weather,
    current_weather: Weather,
    remaining_rain_transition_ticks: i32,
    remaining_thunder_transition_ticks: i32,
) -> Weather {
    let rain_level = current_weather.rain_level()
        + (target_weather.rain_level() - current_weather.rain_level())
            * (1.0 / remaining_rain_transition_ticks.max(1) as f32);
    let thunder_level = current_weather.thunder_level()
        + (target_weather.thunder_level() - current_weather.thunder_level())
            * (1.0 / remaining_thunder_transition_ticks.max(1) as f32);
    Weather::from_valid_levels(rain_level, thunder_level)
}
