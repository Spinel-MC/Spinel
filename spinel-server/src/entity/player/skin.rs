use spinel_core::network::clientbound::play::player_info_update::PlayerInfoProperty;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlayerSkin {
    textures: String,
    signature: Option<String>,
}

impl PlayerSkin {
    pub fn new(textures: impl Into<String>, signature: Option<String>) -> Self {
        Self {
            textures: textures.into(),
            signature,
        }
    }

    pub fn textures(&self) -> &str {
        &self.textures
    }

    pub fn signature(&self) -> Option<&str> {
        self.signature.as_deref()
    }

    pub fn property(&self) -> PlayerInfoProperty {
        PlayerInfoProperty {
            name: "textures".to_string(),
            value: self.textures.clone(),
            signature: self.signature.clone(),
        }
    }
}
