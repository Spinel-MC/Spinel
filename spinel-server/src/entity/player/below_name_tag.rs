use spinel_core::network::clientbound::play::display_scoreboard::DisplayScoreboardPacket;
use spinel_core::network::clientbound::play::scoreboard_objective::{
    ObjectiveRenderType, ScoreboardObjectivePacket,
};
use spinel_utils::component::text::TextComponent;

#[derive(Clone, Debug, PartialEq)]
pub struct BelowNameTag {
    objective_name: String,
    value: TextComponent,
}

impl BelowNameTag {
    pub fn new(name: impl Into<String>, value: TextComponent) -> Self {
        Self {
            objective_name: format!("bnt-{}", name.into()),
            value,
        }
    }

    pub fn get_objective_name(&self) -> &str {
        &self.objective_name
    }

    pub fn get_value(&self) -> &TextComponent {
        &self.value
    }

    pub fn create_packet(&self) -> ScoreboardObjectivePacket {
        ScoreboardObjectivePacket::create(
            self.objective_name.clone(),
            self.value.clone(),
            ObjectiveRenderType::Integer,
        )
    }

    pub fn get_display_packet(&self) -> DisplayScoreboardPacket {
        DisplayScoreboardPacket::below_name(self.objective_name.clone())
    }

    pub fn remove_packet(&self) -> ScoreboardObjectivePacket {
        ScoreboardObjectivePacket::remove(self.objective_name.clone())
    }
}
