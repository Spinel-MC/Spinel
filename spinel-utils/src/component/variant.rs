use crate::component::text::TextComponent;

#[derive(Clone, Debug, PartialEq, Default)]
pub enum ComponentType {
    #[default]
    Empty,
    Text(String),
    Translatable {
        key: String,
        fallback: Option<String>,
        args: Vec<TextComponent>,
    },
    Score {
        name: String,
        objective: String,
        value: Option<String>,
    },
    Selector {
        selector: String,
        separator: Option<Box<TextComponent>>,
    },
    Keybind(String),
    Nbt {
        nbt_path: String,
        source: NbtSource,
        interpret: bool,
        separator: Option<Box<TextComponent>>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum NbtSource {
    Block(String),
    Entity(String),
    Storage(String),
}
