use thiserror::Error;

#[derive(Clone, Debug, Error, PartialEq, Eq)]
#[error("{message}")]
pub struct ArgumentError {
    message: String,
    input: String,
    error_code: i32,
}

impl ArgumentError {
    pub fn new(message: impl Into<String>, input: impl Into<String>, error_code: i32) -> Self {
        Self {
            message: message.into(),
            input: input.into(),
            error_code,
        }
    }

    pub fn get_message(&self) -> &str {
        &self.message
    }
    pub fn get_input(&self) -> &str {
        &self.input
    }
    pub const fn get_error_code(&self) -> i32 {
        self.error_code
    }
}
