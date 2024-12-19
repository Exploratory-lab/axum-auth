//! Error handling module
//!
//! This module contains error handling logic for the application.
//! It defines an `AppError` enum and an `ErrorData` struct
//! that are used to build new errors in the app.

// std library imports
use std::error;
use std::fmt;

/// Application error struct.
///
/// Struct represents an error in the application
/// with its kind, message and source.
///
/// # Examples
/// ```
/// use axum_auth::err::{AppError, ErrorKind};
///
/// let err = AppError { kind: ErrorKind::Env,
///                      message: "Error loading environment variables".to_string(),
///                      source: None
///                     };
/// ```
#[derive(Debug)]
pub struct AppError {
    pub kind: ErrorKind,
    pub message: String,
    pub source: Option<Box<dyn error::Error + 'static>>,
}

/// Implementation block for `AppError` struct.
impl AppError {
    // TODO: Create tests for this function
    /// Creates a new `AppError` instance.
    ///
    /// Function creates a new `AppError` instance with
    /// specified kind, message and source.
    ///
    /// # Examples
    /// ```
    /// use axum_auth::err::{AppError, ErrorKind};
    ///
    /// let err_msg = "Error loading environment variables".to_string();
    ///
    /// let err = AppError::new(ErrorKind::Env,
    ///                         err_msg.clone(),
    ///                         None);
    /// let expected = AppError { kind: ErrorKind::Env, message: err_msg, source: None };
    ///
    /// assert_eq!(err, expected);
    /// ```
    ///
    /// # Parameters
    /// - `kind`: Error kind.
    /// - `message`: Error message.
    /// - `source`: Error source, original error.
    ///
    /// # Returns
    /// - New `AppError` instance.
    pub fn new(
        kind: ErrorKind,
        message: String,
        source: Option<Box<dyn error::Error + 'static>>,
    ) -> Self {
        AppError {
            kind,
            message,
            source,
        }
    }
}

/// Implementation of `PartialEq` trait for `AppError` struct.
impl PartialEq for AppError {
    // TODO: Create tests for this function
    /// Compares two `AppError` instances.
    /// Function compares two `AppError` instances by
    /// comparing their kind, message and source.
    /// If all fields are equal, function returns `true`
    /// otherwise, it returns `false`.
    ///
    /// # Examples
    /// ```
    /// use axum_auth::err::{AppError, ErrorKind};
    ///
    /// let err_msg = "Error loading environment variables".to_string();
    ///
    /// let err1 = AppError::new(ErrorKind::Env,
    ///                          err_msg.clone(),
    ///                          None);
    /// let err2 = AppError::new(ErrorKind::Env,
    ///                          err_msg,
    ///                          None);
    ///
    /// assert_eq!(err1, err2);
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
            && self.message == other.message
            && self.source.as_ref().map(|e| e.to_string())
                == other.source.as_ref().map(|e| e.to_string())
    }
}

/// Implementation of `Display` trait for `AppError` struct.
impl fmt::Display for AppError {
    // TODO: Create tests for this function
    /// Formats `AppError` struct for display.
    ///
    /// Function formats `AppError` struct for display
    /// by printing its kind, message and source.
    ///
    /// # Examples
    /// ```
    /// use axum_auth::err::{AppError, ErrorKind};
    ///
    /// let err_msg = "Error loading environment variables".to_string();
    ///
    /// let err = AppError::new(ErrorKind::Env,
    ///                         err_msg.clone(),
    ///                         None);
    /// let expected = format!("AppError {{ kind: {:?}, message: {}, source: {:?} }}",
    ///                       ErrorKind::Env, err_msg, None::<Box<dyn std::error::Error>>);
    /// let result = format!("{}", err);
    ///
    /// assert_eq!(result, expected);
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "AppError {{ kind: {:?}, message: {}, source: {:?} }}",
            self.kind, self.message, self.source
        )
    }
}

/// Implementation of `Error` trait for `AppError` struct.
impl error::Error for AppError {}

/// Error kind enum.
///
/// Enum represents different kinds of `AppError`.
///
/// # Examples
/// ```
/// use axum_auth::err::ErrorKind;
///
/// let kind = ErrorKind::Env;
/// ```
///
/// # Variants
/// - `Env`: Error setting up application environment.
#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    Env,
}
