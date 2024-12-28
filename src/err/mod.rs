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
    /// Compares two `AppError` instances.
    /// Function compares two `AppError` instances by
    /// comparing their kind, message and source.
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
    /// ```
    ///
    /// # Parameters
    /// - `other`: Another `AppError` instance to compare.
    ///
    /// # Returns
    /// - `true`: If the two instances are equal.
    /// - `false`: If the two instances are not equal.
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
            && self.message == other.message
            && self.source.as_ref().map(|e| e.to_string())
                == other.source.as_ref().map(|e| e.to_string())
    }
}

/// Implementation of `Display` trait for `AppError` struct.
impl fmt::Display for AppError {
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
    ///
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
/// - `Parse`: Error parsing data.
#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    Env,
    Parse,
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests raw `AppError` struct creation.
    #[test]
    fn test_create_app_error() {
        let err = AppError {
            kind: ErrorKind::Env,
            message: "Error loading environment variables".to_string(),
            source: None,
        };
        assert_eq!(err.kind, ErrorKind::Env);
        assert_eq!(err.message, "Error loading environment variables");
        assert!(err.source.is_none());
    }

    // Tests `AppError` struct with source error.
    #[test]
    fn test_app_error_with_source() {
        let source_err: AppError = AppError {
            kind: ErrorKind::Env,
            message: "Some env error".to_string(),
            source: None,
        };
        let err: AppError = AppError {
            kind: ErrorKind::Env,
            message: "Error loading environment variables".to_string(),
            source: Some(Box::new(source_err)),
        };

        assert_eq!(err.kind, ErrorKind::Env);
        assert_eq!(err.message, "Error loading environment variables");
        assert!(err.source.is_some());
    }

    // Tests `AppError` struct new method.
    #[test]
    fn test_app_error_new() {
        let err_msg: String = "Error loading environment variables".to_string();

        let err: AppError = AppError::new(ErrorKind::Env, err_msg.clone(), None);
        let expected: AppError = AppError {
            kind: ErrorKind::Env,
            message: err_msg,
            source: None,
        };

        assert_eq!(err, expected);
    }

    // Tests `AppError` struct new method with source error.
    #[test]
    fn test_app_error_new_with_source() {
        let source_err: AppError = AppError {
            kind: ErrorKind::Env,
            message: "Some env error".to_string(),
            source: None,
        };
        let source_err_copy: AppError = AppError {
            kind: ErrorKind::Env,
            message: "Some env error".to_string(),
            source: None,
        };
        let err_msg: String = "Error loading environment variables".to_string();

        let err: AppError =
            AppError::new(ErrorKind::Env, err_msg.clone(), Some(Box::new(source_err)));
        let expected: AppError = AppError {
            kind: ErrorKind::Env,
            message: err_msg,
            source: Some(Box::new(source_err_copy)),
        };

        assert_eq!(err, expected);
    }

    // Tests `AppError` struct equality.
    #[test]
    fn test_app_error_eq() {
        let err_msg: String = "Error loading environment variables".to_string();

        let err1: AppError = AppError::new(ErrorKind::Env, err_msg.clone(), None);
        let err2: AppError = AppError::new(ErrorKind::Env, err_msg, None);

        assert_eq!(err1, err2);
    }

    // Tests `AppError` struct inequality.
    #[test]
    fn test_app_error_eq_false() {
        let err_msg1: String = "Error loading environment variables".to_string();
        let err_msg2: String = "Error parsing environment variables".to_string();

        let err1: AppError = AppError::new(ErrorKind::Env, err_msg1.clone(), None);
        let err2: AppError = AppError::new(ErrorKind::Parse, err_msg2, None);

        assert_ne!(err1, err2);
    }

    // Tests `AppError` struct display.
    #[test]
    fn test_app_error_display() {
        let err_msg: String = "Error loading environment variables".to_string();

        let err: AppError = AppError::new(ErrorKind::Env, err_msg.clone(), None);
        let expected: String = format!(
            "AppError {{ kind: {:?}, message: {}, source: {:?} }}",
            ErrorKind::Env,
            err_msg,
            None::<Box<dyn std::error::Error>>
        );
        let result = format!("{}", err);

        assert_eq!(result, expected);
    }

    // Tests `ErrorKind` enum equality.
    #[test]
    fn test_error_kind_eq() {
        let kind1: ErrorKind = ErrorKind::Env;
        let kind2: ErrorKind = ErrorKind::Env;

        assert_eq!(kind1, kind2);
    }

    // Tests `ErrorKind` enum inequality.
    #[test]
    fn test_error_kind_eq_false() {
        let kind1: ErrorKind = ErrorKind::Env;
        let kind2: ErrorKind = ErrorKind::Parse;

        assert_ne!(kind1, kind2);
    }
}
