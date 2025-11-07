use crate::component::{
    builder::ComponentBuilder, style::Style, text::TextComponent, variant::ComponentType,
};

pub mod builder;
pub mod color;
pub mod style;
pub mod text;
pub mod variant;

//TODO: figure out if I wanna move this to ./builder.rs
pub struct Component {}

impl Component {
    pub fn empty() -> TextComponent {
        TextComponent {
            component_type: ComponentType::Empty,
            style: Style::empty(),
            children: Vec::new(),
        }
    }

    pub fn text<S: Into<String>>(content: S) -> ComponentBuilder {
        ComponentBuilder {
            component_type: ComponentType::Text(content.into()),
            style: Style::empty(),
            children: Vec::new(),
        }
    }
}
