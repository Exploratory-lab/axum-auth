use std::fmt;

#[derive(Debug)]
pub enum AppError {
    EnvError(String),
    ValidationError(String),
    DatabaseError(String),
    GeneralError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::EnvError(msg) => write!(f, "Environment error: {}", msg),
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            AppError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            AppError::GeneralError(msg) => write!(f, "General error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}
