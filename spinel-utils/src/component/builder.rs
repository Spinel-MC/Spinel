use crate::component::{
    color::TextColor, style::Style, text::TextComponent, variant::ComponentType,
};

#[derive(Clone, Debug, Default)]
pub struct ComponentBuilder {
    pub content: ComponentType<String>,
    pub style: Style,
    pub extra: Vec<TextComponent>,
}

impl ComponentBuilder {
    pub fn new(text: String) -> Self {
        Self {
            content: ComponentType::Text(text),
            style: Style::empty(),
            extra: Vec::new(),
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
        self.extra.push(child);
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
            content: builder.content,
            style: builder.style,
            extra: builder.extra,
        }
    }
}

impl<'a> From<&'a ComponentBuilder> for TextComponent {
    fn from(builder_ref: &'a ComponentBuilder) -> Self {
        TextComponent::from(builder_ref.clone())
    }
}
