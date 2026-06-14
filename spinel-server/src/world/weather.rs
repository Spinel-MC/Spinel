use spinel_core::network::clientbound::play::game_event::{GameEvent, GameEventPacket};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Weather {
    rain_level: f32,
    thunder_level: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WeatherCreationError {
    RainLevelOutsideRange { rain_level: f32 },
    ThunderLevelOutsideRange { thunder_level: f32 },
}

impl fmt::Display for WeatherCreationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RainLevelOutsideRange { rain_level } => {
                write!(
                    formatter,
                    "Rain level must be between 0 and 1: {rain_level}"
                )
            }
            Self::ThunderLevelOutsideRange { thunder_level } => {
                write!(
                    formatter,
                    "Thunder level must be between 0 and 1: {thunder_level}"
                )
            }
        }
    }
}

impl std::error::Error for WeatherCreationError {}

impl Weather {
    pub const CLEAR: Self = Self::from_valid_levels(0.0, 0.0);
    pub const RAIN: Self = Self::from_valid_levels(1.0, 0.0);
    pub const THUNDER: Self = Self::from_valid_levels(1.0, 1.0);

    pub fn new(rain_level: f32, thunder_level: f32) -> Result<Self, WeatherCreationError> {
        if !(0.0..=1.0).contains(&rain_level) {
            return Err(WeatherCreationError::RainLevelOutsideRange { rain_level });
        }
        if !(0.0..=1.0).contains(&thunder_level) {
            return Err(WeatherCreationError::ThunderLevelOutsideRange { thunder_level });
        }
        Ok(Self::from_valid_levels(rain_level, thunder_level))
    }

    pub(crate) const fn from_valid_levels(rain_level: f32, thunder_level: f32) -> Self {
        Self {
            rain_level,
            thunder_level,
        }
    }

    pub const fn rain_level(&self) -> f32 {
        self.rain_level
    }

    pub const fn thunder_level(&self) -> f32 {
        self.thunder_level
    }

    pub fn with_rain_level(self, rain_level: f32) -> Result<Self, WeatherCreationError> {
        Self::new(rain_level, self.thunder_level)
    }

    pub fn with_thunder_level(self, thunder_level: f32) -> Result<Self, WeatherCreationError> {
        Self::new(self.rain_level, thunder_level)
    }

    pub fn map_rain_level(
        self,
        operator: impl FnOnce(f32) -> f32,
    ) -> Result<Self, WeatherCreationError> {
        self.with_rain_level(operator(self.rain_level))
    }

    pub fn map_thunder_level(
        self,
        operator: impl FnOnce(f32) -> f32,
    ) -> Result<Self, WeatherCreationError> {
        self.with_rain_level(operator(self.thunder_level))
    }

    pub fn is_raining(&self) -> bool {
        self.rain_level > 0.0
    }

    pub fn has_rain(&self) -> bool {
        self.is_raining()
    }

    pub fn is_raining_packet(self) -> GameEventPacket {
        let rain_event = if self.is_raining() {
            GameEvent::BeginRaining
        } else {
            GameEvent::EndRaining
        };
        GameEventPacket::from(rain_event)
    }

    pub fn rain_level_packet(self) -> GameEventPacket {
        GameEventPacket::from(GameEvent::RainLevelChange(self.rain_level))
    }

    pub fn thunder_level_packet(self) -> GameEventPacket {
        GameEventPacket::from(GameEvent::ThunderLevelChange(self.thunder_level))
    }

    pub fn packets(self) -> [GameEventPacket; 3] {
        [
            self.is_raining_packet(),
            self.rain_level_packet(),
            self.thunder_level_packet(),
        ]
    }
}
