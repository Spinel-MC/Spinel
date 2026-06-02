use spinel_core::network::clientbound::play::set_player_team::{
    SetPlayerTeamPacket, TeamAction, TeamCollisionRule, TeamNameTagVisibility, TeamParameters,
};
use spinel_utils::component::text::TextComponent;
use std::collections::BTreeSet;

#[derive(Clone, Debug, PartialEq)]
pub struct Team {
    name: String,
    display_name: TextComponent,
    friendly_flags: u8,
    name_tag_visibility: TeamNameTagVisibility,
    collision_rule: TeamCollisionRule,
    color: i32,
    prefix: TextComponent,
    suffix: TextComponent,
    members: BTreeSet<String>,
}

impl Team {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            display_name: TextComponent::empty(),
            friendly_flags: 0,
            name_tag_visibility: TeamNameTagVisibility::Always,
            collision_rule: TeamCollisionRule::Always,
            color: 15,
            prefix: TextComponent::empty(),
            suffix: TextComponent::empty(),
            members: BTreeSet::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn display_name(&self) -> &TextComponent {
        &self.display_name
    }

    pub fn set_display_name(&mut self, display_name: TextComponent) {
        self.display_name = display_name;
    }

    pub const fn friendly_flags(&self) -> u8 {
        self.friendly_flags
    }

    pub fn set_friendly_flags(&mut self, friendly_flags: u8) {
        self.friendly_flags = friendly_flags;
    }

    pub const fn name_tag_visibility(&self) -> TeamNameTagVisibility {
        self.name_tag_visibility
    }

    pub fn set_name_tag_visibility(&mut self, name_tag_visibility: TeamNameTagVisibility) {
        self.name_tag_visibility = name_tag_visibility;
    }

    pub const fn collision_rule(&self) -> TeamCollisionRule {
        self.collision_rule
    }

    pub fn set_collision_rule(&mut self, collision_rule: TeamCollisionRule) {
        self.collision_rule = collision_rule;
    }

    pub const fn color(&self) -> i32 {
        self.color
    }

    pub fn set_color(&mut self, color: i32) {
        self.color = color;
    }

    pub fn prefix(&self) -> &TextComponent {
        &self.prefix
    }

    pub fn set_prefix(&mut self, prefix: TextComponent) {
        self.prefix = prefix;
    }

    pub fn suffix(&self) -> &TextComponent {
        &self.suffix
    }

    pub fn set_suffix(&mut self, suffix: TextComponent) {
        self.suffix = suffix;
    }

    pub fn members(&self) -> impl Iterator<Item = &str> {
        self.members.iter().map(String::as_str)
    }

    pub fn create_packet(&self) -> SetPlayerTeamPacket {
        SetPlayerTeamPacket {
            team_name: self.name.clone(),
            action: TeamAction::Create(self.parameters()),
        }
    }

    pub fn remove_packet(&self) -> SetPlayerTeamPacket {
        SetPlayerTeamPacket {
            team_name: self.name.clone(),
            action: TeamAction::Remove,
        }
    }

    pub fn update_packet(&self) -> SetPlayerTeamPacket {
        SetPlayerTeamPacket {
            team_name: self.name.clone(),
            action: TeamAction::Update(self.parameters_without_members()),
        }
    }

    pub fn add_member(&mut self, member: impl Into<String>) -> Option<SetPlayerTeamPacket> {
        let member = member.into();
        if !self.members.insert(member.clone()) {
            return None;
        }
        Some(SetPlayerTeamPacket {
            team_name: self.name.clone(),
            action: TeamAction::AddEntities(vec![member]),
        })
    }

    pub fn remove_member(&mut self, member: &str) -> Option<SetPlayerTeamPacket> {
        if !self.members.remove(member) {
            return None;
        }
        Some(SetPlayerTeamPacket {
            team_name: self.name.clone(),
            action: TeamAction::RemoveEntities(vec![member.to_owned()]),
        })
    }

    fn parameters(&self) -> TeamParameters {
        let mut parameters = self.parameters_without_members();
        parameters.entities = self.members().map(str::to_owned).collect();
        parameters
    }

    fn parameters_without_members(&self) -> TeamParameters {
        TeamParameters {
            display_name: self.display_name.clone(),
            friendly_flags: self.friendly_flags,
            name_tag_visibility: self.name_tag_visibility,
            collision_rule: self.collision_rule,
            color: self.color,
            prefix: self.prefix.clone(),
            suffix: self.suffix.clone(),
            entities: Vec::new(),
        }
    }
}
