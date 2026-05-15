#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RespawnScreenState {
    Enabled,
    ImmediateRespawn,
}

impl RespawnScreenState {
    pub(crate) const fn value(self) -> f32 {
        match self {
            Self::Enabled => 0.0,
            Self::ImmediateRespawn => 1.0,
        }
    }
}
