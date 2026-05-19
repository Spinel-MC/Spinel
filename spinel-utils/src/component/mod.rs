use crate::component::{
    builder::ComponentBuilder, style::Style, text::TextComponent, variant::ComponentType,
};

pub mod builder;
mod codec;
pub mod color;
mod decode;
pub mod events;
pub mod nbt;
pub mod style;
#[cfg(test)]
mod tests;
pub mod text;
pub mod variant;

pub struct Component {}

impl Component {
    pub fn empty() -> TextComponent {
        TextComponent {
            content: ComponentType::Empty,
            style: Style::empty(),
            extra: Vec::new(),
        }
    }

    pub fn text<S: Into<String>>(content: S) -> ComponentBuilder {
        ComponentBuilder {
            content: ComponentType::Text(content.into()),
            style: Style::empty(),
            extra: Vec::new(),
        }
    }
}
