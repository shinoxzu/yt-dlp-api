use serde::Serialize;
use validator::ValidationErrors;

#[derive(Serialize, Clone)]
pub struct ErrorDTO {
    pub message: String,
}

impl ErrorDTO {
    pub fn new(message: impl Into<String>) -> Self {
        ErrorDTO {
            message: message.into(),
        }
    }
}

#[derive(Serialize, Clone)]
pub struct ErrorWithValidationErrorsDTO {
    pub message: String,
    pub errors: ValidationErrors,
}

impl ErrorWithValidationErrorsDTO {
    pub fn new(message: String, errors: ValidationErrors) -> Self {
        ErrorWithValidationErrorsDTO { message, errors }
    }
}
