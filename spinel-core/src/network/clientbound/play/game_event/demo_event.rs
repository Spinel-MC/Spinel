#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DemoEvent {
    ShowWelcomeScreen,
    ShowMovementControls,
    ShowJumpControl,
    ShowInventoryControl,
    ShowDemoEndedScreen,
}

impl DemoEvent {
    pub(crate) const fn value(self) -> f32 {
        match self {
            Self::ShowWelcomeScreen => 0.0,
            Self::ShowMovementControls => 101.0,
            Self::ShowJumpControl => 102.0,
            Self::ShowInventoryControl => 103.0,
            Self::ShowDemoEndedScreen => 104.0,
        }
    }
}
