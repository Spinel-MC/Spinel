use crate::data_components::DataComponentValue;
use crate::data_components::nbt_reader::{
    bool_field_or, compound_from_nbt, i32_field_or, string_field,
};
use spinel_nbt::{Nbt, NbtCompound};
use spinel_utils::component::text::TextComponent;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FilteredString {
    text: String,
    filtered: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FilteredComponent {
    text: TextComponent,
    filtered: Option<TextComponent>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct WritableBookContent {
    pages: Vec<FilteredString>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WrittenBookContent {
    title: FilteredString,
    author: String,
    generation: i32,
    pages: Vec<FilteredComponent>,
    resolved: bool,
}

impl FilteredString {
    #[must_use]
    pub fn new(text: String, filtered: Option<String>) -> Self {
        Self { text, filtered }
    }

    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    #[must_use]
    pub fn filtered(&self) -> Option<&str> {
        self.filtered.as_deref()
    }
}

impl FilteredComponent {
    #[must_use]
    pub fn new(text: TextComponent, filtered: Option<TextComponent>) -> Self {
        Self { text, filtered }
    }

    #[must_use]
    pub fn text(&self) -> &TextComponent {
        &self.text
    }

    #[must_use]
    pub fn filtered(&self) -> Option<&TextComponent> {
        self.filtered.as_ref()
    }
}

impl WritableBookContent {
    #[must_use]
    pub fn new(pages: Vec<FilteredString>) -> Self {
        Self { pages }
    }

    #[must_use]
    pub fn pages(&self) -> &[FilteredString] {
        &self.pages
    }
}

impl WrittenBookContent {
    #[must_use]
    pub fn new(
        title: FilteredString,
        author: String,
        generation: i32,
        pages: Vec<FilteredComponent>,
        resolved: bool,
    ) -> Self {
        Self {
            title,
            author,
            generation,
            pages,
            resolved,
        }
    }

    #[must_use]
    pub fn title(&self) -> &FilteredString {
        &self.title
    }

    #[must_use]
    pub fn author(&self) -> &str {
        &self.author
    }

    #[must_use]
    pub const fn generation(&self) -> i32 {
        self.generation
    }

    #[must_use]
    pub fn pages(&self) -> &[FilteredComponent] {
        &self.pages
    }

    #[must_use]
    pub const fn resolved(&self) -> bool {
        self.resolved
    }
}

impl DataComponentValue for WritableBookContent {
    fn to_component_nbt(&self) -> Nbt {
        let mut compound = NbtCompound::new();
        if !self.pages.is_empty() {
            compound.insert(
                "pages".to_string(),
                Nbt::List(
                    self.pages
                        .iter()
                        .map(FilteredString::to_nbt)
                        .collect::<Vec<_>>()
                        .into_boxed_slice(),
                ),
            );
        }
        Nbt::Compound(compound)
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        let compound = compound_from_nbt(component_nbt)?;
        let pages = match compound.get("pages") {
            Some(Nbt::List(pages)) => pages
                .iter()
                .map(FilteredString::from_nbt)
                .collect::<Option<Vec<_>>>()?,
            None => Vec::new(),
            _ => return None,
        };
        Some(Self { pages })
    }
}

impl DataComponentValue for WrittenBookContent {
    fn to_component_nbt(&self) -> Nbt {
        let mut compound = NbtCompound::new();
        compound.insert("title".to_string(), self.title.to_nbt());
        compound.insert("author".to_string(), Nbt::String(self.author.clone()));
        if self.generation != 0 {
            compound.insert("generation".to_string(), Nbt::Int(self.generation));
        }
        if !self.pages.is_empty() {
            compound.insert(
                "pages".to_string(),
                Nbt::List(
                    self.pages
                        .iter()
                        .map(FilteredComponent::to_nbt)
                        .collect::<Vec<_>>()
                        .into_boxed_slice(),
                ),
            );
        }
        if self.resolved {
            compound.insert("resolved".to_string(), Nbt::Byte(1));
        }
        Nbt::Compound(compound)
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        let compound = compound_from_nbt(component_nbt)?;
        let pages = match compound.get("pages") {
            Some(Nbt::List(pages)) => pages
                .iter()
                .map(FilteredComponent::from_nbt)
                .collect::<Option<Vec<_>>>()?,
            None => Vec::new(),
            _ => return None,
        };
        Some(Self {
            title: FilteredString::from_nbt(compound.get("title")?)?,
            author: string_field(compound, "author")?,
            generation: i32_field_or(compound, "generation", 0)?,
            pages,
            resolved: bool_field_or(compound, "resolved", false)?,
        })
    }
}

impl FilteredString {
    fn to_nbt(&self) -> Nbt {
        let mut compound = NbtCompound::new();
        compound.insert("raw".to_string(), Nbt::String(self.text.clone()));
        if let Some(filtered) = &self.filtered {
            compound.insert("filtered".to_string(), Nbt::String(filtered.clone()));
        }
        Nbt::Compound(compound)
    }

    fn from_nbt(component_nbt: &Nbt) -> Option<Self> {
        let compound = compound_from_nbt(component_nbt)?;
        let filtered = match compound.get("filtered") {
            Some(Nbt::String(filtered)) => Some(filtered.clone()),
            None => None,
            _ => return None,
        };
        Some(Self {
            text: string_field(compound, "raw")?,
            filtered,
        })
    }
}

impl FilteredComponent {
    fn to_nbt(&self) -> Nbt {
        let mut compound = NbtCompound::new();
        compound.insert("raw".to_string(), self.text.to_nbt_compound());
        if let Some(filtered) = &self.filtered {
            compound.insert("filtered".to_string(), filtered.to_nbt_compound());
        }
        Nbt::Compound(compound)
    }

    fn from_nbt(component_nbt: &Nbt) -> Option<Self> {
        let compound = compound_from_nbt(component_nbt)?;
        let filtered = match compound.get("filtered") {
            Some(filtered) => Some(text_component_from_nbt(filtered)?),
            None => None,
        };
        Some(Self {
            text: text_component_from_nbt(compound.get("raw")?)?,
            filtered,
        })
    }
}

fn text_component_from_nbt(component_nbt: &Nbt) -> Option<TextComponent> {
    match component_nbt {
        Nbt::String(value) => serde_json::from_str(value)
            .ok()
            .or_else(|| Some(TextComponent::literal(value.clone()))),
        Nbt::Compound(compound) => {
            let json =
                serde_json::Value::Object(spinel_nbt::nbt_compound_to_json(compound.clone()));
            serde_json::from_value(json).ok()
        }
        _ => None,
    }
}
