use spinel_core::network::clientbound::play::boss_bar::{
    BossBarAction, BossBarColor, BossBarOverlay, BossBarPacket,
};
use spinel_utils::component::text::TextComponent;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub struct BossBar {
    id: Uuid,
    title: TextComponent,
    health: f32,
    color: BossBarColor,
    overlay: BossBarOverlay,
    flags: u8,
}

impl BossBar {
    pub fn new(
        title: TextComponent,
        health: f32,
        color: BossBarColor,
        overlay: BossBarOverlay,
        flags: u8,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            health,
            color,
            overlay,
            flags,
        }
    }

    pub const fn id(&self) -> Uuid {
        self.id
    }

    pub fn title(&self) -> &TextComponent {
        &self.title
    }

    pub const fn health(&self) -> f32 {
        self.health
    }

    pub const fn color(&self) -> BossBarColor {
        self.color
    }

    pub const fn overlay(&self) -> BossBarOverlay {
        self.overlay
    }

    pub const fn flags(&self) -> u8 {
        self.flags
    }

    pub fn add_packet(&self) -> BossBarPacket {
        BossBarPacket {
            id: self.id,
            action: BossBarAction::Add {
                title: self.title.clone(),
                health: self.health,
                color: self.color,
                overlay: self.overlay,
                flags: self.flags,
            },
        }
    }

    pub fn remove_packet(&self) -> BossBarPacket {
        BossBarPacket {
            id: self.id,
            action: BossBarAction::Remove,
        }
    }
}

pub use spinel_core::network::clientbound::play::boss_bar::{
    BossBarColor as WorldBossBarColor, BossBarOverlay as WorldBossBarOverlay,
};
