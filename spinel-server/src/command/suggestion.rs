use crate::command::{CommandConditionContext, CommandContext};
use crate::server::MinecraftServer;

pub type SuggestionCallbackFunction =
    fn(Option<&MinecraftServer>, CommandConditionContext, &CommandContext, &mut Suggestion);

pub type SuggestionCallbackMethod<T> =
    fn(&T, Option<&MinecraftServer>, CommandConditionContext, &CommandContext, &mut Suggestion);

type ErasedSuggestionCallback = unsafe fn(
    usize,
    usize,
    Option<&MinecraftServer>,
    CommandConditionContext,
    &CommandContext,
    &mut Suggestion,
);

#[derive(Clone, Copy)]
pub struct SuggestionCallback {
    receiver: usize,
    callback: usize,
    dispatch: ErasedSuggestionCallback,
}

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

impl SuggestionCallback {
    pub fn from_function(callback: SuggestionCallbackFunction) -> Self {
        Self {
            receiver: 0,
            callback: callback as usize,
            dispatch: dispatch_suggestion_function,
        }
    }

    pub fn from_method<T>(receiver: &T, callback: SuggestionCallbackMethod<T>) -> Self {
        Self {
            receiver: receiver as *const T as usize,
            callback: callback as usize,
            dispatch: dispatch_suggestion_method::<T>,
        }
    }

    pub fn suggest(
        self,
        server: Option<&MinecraftServer>,
        condition_context: CommandConditionContext,
        context: &CommandContext,
        suggestion: &mut Suggestion,
    ) {
        unsafe {
            (self.dispatch)(
                self.receiver,
                self.callback,
                server,
                condition_context,
                context,
                suggestion,
            );
        }
    }
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

unsafe fn dispatch_suggestion_function(
    _receiver: usize,
    callback: usize,
    server: Option<&MinecraftServer>,
    condition_context: CommandConditionContext,
    context: &CommandContext,
    suggestion: &mut Suggestion,
) {
    let callback: SuggestionCallbackFunction = unsafe { std::mem::transmute(callback) };
    callback(server, condition_context, context, suggestion);
}

unsafe fn dispatch_suggestion_method<T>(
    receiver: usize,
    callback: usize,
    server: Option<&MinecraftServer>,
    condition_context: CommandConditionContext,
    context: &CommandContext,
    suggestion: &mut Suggestion,
) {
    let receiver = unsafe { &*(receiver as *const T) };
    let callback: SuggestionCallbackMethod<T> = unsafe { std::mem::transmute(callback) };
    callback(receiver, server, condition_context, context, suggestion);
}
