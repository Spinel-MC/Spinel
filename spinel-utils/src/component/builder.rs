
use crate::component::{color::TextColor, style::Style, text::TextComponent, variant::ComponentType};

#[derive(Clone, Debug, Default)]
pub struct ComponentBuilder {
    pub component_type: ComponentType,
    pub style: Style,
    pub children: Vec<TextComponent>,
}

impl ComponentBuilder {
    pub fn new(text: String) -> Self {
        Self {
            component_type: ComponentType::Text(text),
            style: Style::empty(),
            children: Vec::new(),
        }
    }

    pub fn color(mut self, color: TextColor) -> Self {
        self.style.color = Some(color);
        self
    }

    pub fn bold(mut self, val: bool) -> Self {
        self.style.bold = Some(val);
        self
    }
    
    pub fn italic(mut self, val: bool) -> Self {
        self.style.italic = Some(val);
        self
    }

    pub fn obfuscated(mut self, val: bool) -> Self {
        self.style.obfuscated = Some(val);
        self
    }

    pub fn strikethrough(mut self, val: bool) -> Self {
        self.style.strikethrough = Some(val);
        self
    }

    pub fn underlined(mut self, val: bool) -> Self {
        self.style.underlined = Some(val);
        self
    }

    pub fn append(mut self, child: TextComponent) -> Self {
        self.children.push(child);
        self
    }

    pub fn to_json_string(&self) -> String {
        TextComponent::from(self.clone()).to_json_string()
    }

    pub fn to_ansi_string(&self) -> String {
        TextComponent::from(self.clone()).to_ansi_string()
    }
}

impl From<ComponentBuilder> for TextComponent {
    fn from(builder: ComponentBuilder) -> Self {
        TextComponent {
            component_type: builder.component_type,
            style: builder.style,
            children: builder.children,
        }
    }
}

impl<'a> From<&'a ComponentBuilder> for TextComponent {
    fn from(builder_ref: &'a ComponentBuilder) -> Self {
        TextComponent::from(builder_ref.clone())
    }
}