impl World {
    pub fn register_scoreboard_team(&mut self, team: Team) -> Result<bool> {
        if self.scoreboard_teams.contains_key(team.name()) {
            return Ok(false);
        }
        self.dispatch_packet_to_entered_players(team.create_packet())?;
        self.scoreboard_teams.insert(team.name().to_owned(), team);
        Ok(true)
    }

    pub fn remove_scoreboard_team(&mut self, team_name: &str) -> Result<bool> {
        let Some(team) = self.scoreboard_teams.remove(team_name) else {
            return Ok(false);
        };
        self.entities.iter_mut().for_each(|entity| match entity {
            Entity::Living(entity) if entity.get_team() == Some(team_name) => {
                entity.set_team(None)
            }
            Entity::Item(_) => {}
            Entity::Player(player) if player.get_team() == Some(team_name) => {
                player.set_scoreboard_team(None, None);
            }
            _ => {}
        });
        self.dispatch_packet_to_entered_players(team.remove_packet())?;
        Ok(true)
    }

    pub fn scoreboard_team(&self, team_name: &str) -> Option<&Team> {
        self.scoreboard_teams.get(team_name)
    }

    pub fn scoreboard_teams(&self) -> impl Iterator<Item = &Team> {
        self.scoreboard_teams.values()
    }

    pub fn set_entity_scoreboard_team(
        &mut self,
        entity_id: EntityId,
        team_name: Option<&str>,
    ) -> Result<bool> {
        let current_team_name = self
            .entity_by_id(entity_id)
            .and_then(entity_scoreboard_team_name)
            .map(str::to_owned);
        let requested_team_name = team_name.map(str::to_owned);
        if current_team_name == requested_team_name {
            return Ok(false);
        }
        if let Some(requested_team_name) = requested_team_name.as_deref() {
            if !self.scoreboard_teams.contains_key(requested_team_name) {
                return Err(Error::new(
                    ErrorKind::NotFound,
                    format!("scoreboard team {requested_team_name} is not registered"),
                ));
            }
        }
        let mut previous_team = current_team_name
            .as_deref()
            .and_then(|team_name| self.scoreboard_teams.remove(team_name));
        let mut requested_team = requested_team_name
            .as_deref()
            .and_then(|team_name| self.scoreboard_teams.remove(team_name));
        let packets = self.apply_entity_scoreboard_team(
            entity_id,
            previous_team.as_mut(),
            requested_team.as_mut(),
        );
        if let Some(previous_team) = previous_team {
            self.scoreboard_teams
                .insert(previous_team.name().to_owned(), previous_team);
        }
        if let Some(requested_team) = requested_team {
            self.scoreboard_teams
                .insert(requested_team.name().to_owned(), requested_team);
        }
        let Some(packets) = packets else {
            return Ok(false);
        };
        packets.into_iter().try_for_each(|packet| {
            self.send_packet_to_player_viewers_and_self(entity_id, packet)
        })?;
        Ok(true)
    }

    fn apply_entity_scoreboard_team(
        &mut self,
        entity_id: EntityId,
        previous_team: Option<&mut Team>,
        requested_team: Option<&mut Team>,
    ) -> Option<Vec<SetPlayerTeamPacket>> {
        let entity = self.entity_by_id_mut(entity_id)?;
        Some(match entity {
            Entity::Creature(entity) => entity.set_scoreboard_team(previous_team, requested_team),
            Entity::ExperienceOrb(_) | Entity::Generic(_) => Vec::new(),
            Entity::Living(entity) => entity.set_scoreboard_team(previous_team, requested_team),
            Entity::Item(_) => Vec::new(),
            Entity::Player(player) => player.set_scoreboard_team(previous_team, requested_team),
            Entity::Projectile(_) => Vec::new(),
        })
    }
}

fn entity_scoreboard_team_name(entity: &Entity) -> Option<&str> {
    match entity {
        Entity::Creature(entity) => entity.get_team(),
        Entity::ExperienceOrb(_) | Entity::Generic(_) => None,
        Entity::Living(entity) => entity.get_team(),
        Entity::Item(_) => None,
        Entity::Player(player) => player.get_team(),
        Entity::Projectile(_) => None,
    }
}
