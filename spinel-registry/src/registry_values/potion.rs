use crate::Identifier;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Potion {
    protocol_id: i32,
    effects: Vec<PotionEffect>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PotionEffect {
    effect: Identifier,
    amplifier: i32,
    duration: i32,
    is_ambient: bool,
    is_visible: bool,
    should_show_icon: bool,
}

impl Potion {
    #[must_use]
    pub fn new(protocol_id: i32, effects: Vec<PotionEffect>) -> Self {
        Self { protocol_id, effects }
    }

    #[must_use]
    pub const fn get_protocol_id(&self) -> i32 {
        self.protocol_id
    }

    #[must_use]
    pub fn get_effects(&self) -> &[PotionEffect] {
        &self.effects
    }
}

impl PotionEffect {
    #[must_use]
    pub fn new(
        effect: Identifier,
        amplifier: i32,
        duration: i32,
        is_ambient: bool,
        is_visible: bool,
        should_show_icon: bool,
    ) -> Self {
        Self { effect, amplifier, duration, is_ambient, is_visible, should_show_icon }
    }

    #[must_use]
    pub const fn get_effect(&self) -> &Identifier {
        &self.effect
    }

    #[must_use]
    pub const fn get_amplifier(&self) -> i32 {
        self.amplifier
    }

    #[must_use]
    pub const fn get_duration(&self) -> i32 {
        self.duration
    }

    #[must_use]
    pub const fn is_ambient(&self) -> bool {
        self.is_ambient
    }

    #[must_use]
    pub const fn is_visible(&self) -> bool {
        self.is_visible
    }

    #[must_use]
    pub const fn should_show_icon(&self) -> bool {
        self.should_show_icon
    }
}