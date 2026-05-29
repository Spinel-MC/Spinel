use crate::command::{CommandContext, CommandSenderKind};

pub type SuggestionCallback = fn(CommandSenderKind, &CommandContext, &mut Suggestion);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Suggestion {
    input: String,
    start: usize,
    length: usize,
    entries: Vec<SuggestionEntry>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SuggestionEntry {
    entry: String,
    tooltip: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SuggestionType {
    AskServer,
    AllRecipes,
    AvailableSounds,
    SummonableEntities,
}

impl Suggestion {
    pub fn new(input: impl Into<String>, start: usize, length: usize) -> Self {
        Self {
            input: input.into(),
            start,
            length,
            entries: Vec::new(),
        }
    }

    pub fn input(&self) -> &str {
        &self.input
    }

    pub const fn start(&self) -> usize {
        self.start
    }

    pub fn set_start(&mut self, start: usize) {
        self.start = start;
    }

    pub const fn length(&self) -> usize {
        self.length
    }

    pub fn set_length(&mut self, length: usize) {
        self.length = length;
    }

    pub fn entries(&self) -> &[SuggestionEntry] {
        &self.entries
    }

    pub fn add_entry(&mut self, entry: SuggestionEntry) {
        self.entries.push(entry);
    }
}

impl SuggestionEntry {
    pub fn new(entry: impl Into<String>) -> Self {
        Self {
            entry: entry.into(),
            tooltip: None,
        }
    }

    pub fn with_tooltip(entry: impl Into<String>, tooltip: impl Into<String>) -> Self {
        Self {
            entry: entry.into(),
            tooltip: Some(tooltip.into()),
        }
    }

    pub fn entry(&self) -> &str {
        &self.entry
    }

    pub fn tooltip(&self) -> Option<&str> {
        self.tooltip.as_deref()
    }
}

impl SuggestionType {
    pub const fn identifier(self) -> &'static str {
        match self {
            Self::AskServer => "minecraft:ask_server",
            Self::AllRecipes => "minecraft:all_recipes",
            Self::AvailableSounds => "minecraft:available_sounds",
            Self::SummonableEntities => "minecraft:summonable_entities",
        }
    }
}
