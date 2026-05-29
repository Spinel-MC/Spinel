use crate::command::CommandSenderKind;

pub type ArgumentCallback = fn(CommandSenderKind, &ArgumentSyntaxException);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArgumentSyntaxException {
    message: String,
    input: String,
    error_code: i32,
}

impl ArgumentSyntaxException {
    pub fn new(message: impl Into<String>, input: impl Into<String>, error_code: i32) -> Self {
        Self {
            message: message.into(),
            input: input.into(),
            error_code,
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn input(&self) -> &str {
        &self.input
    }

    pub const fn error_code(&self) -> i32 {
        self.error_code
    }
}
