use crate::entity::game_mode::GameMode;
use crate::network::clientbound::play::game_event::{
    DemoEvent, LimitedCraftingState, RespawnScreenState, WinGameReason,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameEvent {
    NoRespawnBlockAvailable,
    BeginRaining,
    EndRaining,
    ChangeGameMode(GameMode),
    WinGame(WinGameReason),
    DemoEvent(DemoEvent),
    ArrowHitPlayer,
    RainLevelChange(f32),
    ThunderLevelChange(f32),
    PlayPufferfishStingSound,
    PlayElderGuardianMobAppearance,
    SetRespawnScreen(RespawnScreenState),
    SetLimitedCrafting(LimitedCraftingState),
    StartWaitingForLevelChunks,
}

impl GameEvent {
    pub const fn event_id(self) -> u8 {
        match self {
            Self::NoRespawnBlockAvailable => 0,
            Self::BeginRaining => 1,
            Self::EndRaining => 2,
            Self::ChangeGameMode(_) => 3,
            Self::WinGame(_) => 4,
            Self::DemoEvent(_) => 5,
            Self::ArrowHitPlayer => 6,
            Self::RainLevelChange(_) => 7,
            Self::ThunderLevelChange(_) => 8,
            Self::PlayPufferfishStingSound => 9,
            Self::PlayElderGuardianMobAppearance => 10,
            Self::SetRespawnScreen(_) => 11,
            Self::SetLimitedCrafting(_) => 12,
            Self::StartWaitingForLevelChunks => 13,
        }
    }

    pub const fn value(self) -> f32 {
        match self {
            Self::NoRespawnBlockAvailable => 0.0,
            Self::BeginRaining => 0.0,
            Self::EndRaining => 0.0,
            Self::ChangeGameMode(game_mode) => game_mode.id() as f32,
            Self::WinGame(reason) => reason.value(),
            Self::DemoEvent(event) => event.value(),
            Self::ArrowHitPlayer => 0.0,
            Self::RainLevelChange(level) => level,
            Self::ThunderLevelChange(level) => level,
            Self::PlayPufferfishStingSound => 0.0,
            Self::PlayElderGuardianMobAppearance => 0.0,
            Self::SetRespawnScreen(state) => state.value(),
            Self::SetLimitedCrafting(state) => state.value(),
            Self::StartWaitingForLevelChunks => 0.0,
        }
    }
}
