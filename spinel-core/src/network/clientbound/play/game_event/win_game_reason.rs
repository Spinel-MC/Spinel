#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WinGameReason {
    JustRespawn,
    ShowCreditsAndRespawn,
}

impl WinGameReason {
    pub(crate) const fn value(self) -> f32 {
        match self {
            Self::JustRespawn => 0.0,
            Self::ShowCreditsAndRespawn => 1.0,
        }
    }
}
