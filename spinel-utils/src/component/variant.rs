use super::text::TextComponent;

#[derive(Clone, Debug)]
pub enum ComponentType<T: Into<String>> {
    Empty,
    Text(T),
    StaticText(&'static str),
    Translatable {
        key: String,
        args: Vec<TextComponent>,
    },
    StaticTranslatable(&'static str),
    Score {
        name: String,
        objective: String,
    },
    Selector(String),
    StaticSelector(&'static str),
    Keybind(String),
    StaticKeybind(&'static str),
    Nbt {
        nbt_path: String,
        source: String,
    },
}

impl<T: Into<String>> Default for ComponentType<T> {
    fn default() -> Self {
        ComponentType::Empty
    }
}
