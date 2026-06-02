use crate::component::color::TextColor;
use crate::component::events::{ClickEvent, HoverEvent};
use crate::component::style::Style;
use crate::component::text::TextComponent;
use crate::component::variant::ComponentType;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ComponentBuilder {
    pub content: ComponentType,
    pub style: Style,
    pub extra: Vec<TextComponent>,
}

impl ComponentBuilder {
    pub fn new(text: impl Into<String>) -> Self {
        Self::from_content(ComponentType::Text(text.into()))
    }

    pub fn from_content(content: ComponentType) -> Self {
        Self {
            content,
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

    pub fn click_event(mut self, event: ClickEvent) -> Self {
        self.style.click_event = Some(event);
        self
    }

    pub fn hover_event(mut self, event: HoverEvent) -> Self {
        self.style.hover_event = Some(event);
        self
    }

    pub fn insertion(mut self, insertion: impl Into<String>) -> Self {
        self.style.insertion = Some(insertion.into());
        self
    }

    pub fn font(mut self, font: impl Into<String>) -> Self {
        self.style.font = Some(font.into());
        self
    }

    pub fn append(mut self, child: impl Into<TextComponent>) -> Self {
        self.extra.push(child.into());
        self
    }

    pub fn argument(mut self, argument: impl Into<TextComponent>) -> Self {
        if let ComponentType::Translatable { args, .. } = &mut self.content {
            args.push(argument.into());
        }
        self
    }

    pub fn build(self) -> TextComponent {
        self.into()
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

impl From<&ComponentBuilder> for TextComponent {
    fn from(builder_ref: &ComponentBuilder) -> Self {
        TextComponent::from(builder_ref.clone())
    }
}
