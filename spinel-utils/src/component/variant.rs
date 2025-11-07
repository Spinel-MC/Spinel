use super::text::TextComponent;

#[derive(Clone, Debug, Default)]
pub enum ComponentType {
    #[default]
    Empty,
    Text(String),
    Translatable {
        key: String,
        args: Vec<TextComponent>,
    },
    Score {
        name: String,
        objective: String,
    },
    Selector(String),
    Keybind(String),
    Nbt {
        nbt_path: String,
        source: String,
    },
}
