//! This module contains the prelude for the library.
//!
//! It contains all the key types and functions that are
//! used throughout the library. This module is meant to
//! be imported in the library root module and then re-exported
//! so that all the types and functions are available to the
//! library users.

use std::error;

use crate::err::{AppError, ErrorKind};

/// Function to parse a string into a u16.
///
/// # Examples
/// ```
/// use axum_auth::prelude::is_u16;
///
/// let value = "123";
/// let result = is_u16(value);
///
/// assert_eq!(result, Ok(123));
/// ```
///
/// # Parameters
/// - `value`: The string slice to parse.
///
/// # Returns
/// - `u16`: The parsed u16 value.
/// - `AppError`: Error parsing the string.
pub fn is_u16(value: &str) -> Result<u16, AppError> {
    match value.parse::<u16>() {
        Ok(v) => Ok(v),
        Err(e) => {
            let kind: ErrorKind = ErrorKind::Env;
            let message: String = format!("Failed to parse value as u16: '{}'", value);
            let source: Option<Box<dyn error::Error>> = Some(Box::new(e));

            Err(AppError::new(kind, message, source))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test checks if the function can parse a valid
    /// u16 value from a string slice.
    #[test]
    fn test_is_u16() {
        let value: &str = "123";
        let result: Result<u16, AppError> = is_u16(value);

        assert_eq!(result, Ok(123));
    }

    /// Test checks if the function returns an error
    /// when the string slice cannot be parsed into
    /// a u16 value.
    #[test]
    fn test_is_u16_invalid() {
        let value: &str = "abc";
        let result: Result<u16, AppError> = is_u16(value);

        assert!(result.is_err());
    }

    /// Test checks if the function returns an error
    /// when the string slice is empty.
    #[test]
    fn test_is_u16_empty() {
        let value: &str = "";
        let result: Result<u16, AppError> = is_u16(value);

        assert!(result.is_err());
    }

    /// Test checks if the function returns an error
    /// when the string slice is a negative number.
    /// Negative numbers are not allowed.
    #[test]
    fn test_is_u16_negative() {
        let value: &str = "-123";
        let result: Result<u16, AppError> = is_u16(value);

        assert!(result.is_err());
    }

    /// Test checks if the function returns an error
    /// when the string slice is a floating point number.
    /// Floating point numbers are not allowed.
    #[test]
    fn test_is_u16_float() {
        let value: &str = "123.45";
        let result: Result<u16, AppError> = is_u16(value);

        assert!(result.is_err());
    }
}
