//! Moudle contains the associated types for the application,
//! so that the values from external sources can be verified
//! for type correctness.

// External imports
use std::{fs, path::Path};

// Internal imports
use super::err::{AppError, ErrorKind};
use crate::strings::err::INVALID_VALUE_FOR_TYPE;

/// ## Environment variable type enum.
///
/// Enum represents the type of the environment variables.
///
/// ## Examples
/// ```
/// use axum_auth::core::types::AppType;
///
/// let type_: AppType = AppType::String;
/// ```
///
/// ## Variants
/// - `String`: String type environment variable.
/// - `U16`: Unsigned 16-bit integer type environment variable.
/// - `Enum`: Enum type environment variable with allowed values.
/// - `FilePath`: File path type environment variable.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppType {
    // General type for any string value, but it must not be empty:
    // "value" & "123"  - valid
    // "" - invalid
    String,

    // Unsigned 16-bit integer type:
    // "123" & "0" & "65535" - valid
    // "65536" & "abc" - invalid
    U16,

    // Enum type with allowed values:
    // Example: Allowed values are ["development", "production"]
    // "development" & "production" - valid
    // "staging" - invalid
    Enum(&'static [&'static str]),

    // File path type:
    // "/path/to/file" - valid
    // "" - invalid
    FilePath,
}

impl AppType {
    /// ## Verifies the value based on the type.
    ///
    /// Function verifies the value based on the
    /// specified type.
    ///
    /// ## Examples
    /// ```
    /// use axum_auth::core::types::AppType;
    ///
    /// let val: &str = "string value";
    /// let result = AppType::String.verify(val);
    ///
    /// assert!(result.is_ok());
    /// ```
    ///
    /// ## Arguments
    /// - `val`: `&str` - Value to verify.
    ///
    /// ## Returns
    /// - `Result<(), AppError>`:
    ///   + `Ok(())`: If the value is valid.
    ///   + `Err(AppError)`: If the value is invalid.
    pub fn verify(&self, val: &str) -> Result<(), AppError> {
        match self {
            Self::String => self.verify_string(val),

            Self::U16 => self.verify_u16(val),

            Self::Enum(allowed_values) => self.verify_enum(allowed_values, val),

            Self::FilePath => self.verify_file_path(val),
        }
    }

    /// ## Verifies the string value.
    ///
    /// Function checks if the string value is not empty.
    ///
    /// ## Arguments
    /// - `val`: `&str` - Value to verify.
    ///
    /// ## Returns
    /// - `Result<(), AppError>`:
    ///    + `Ok(())`: If the value is valid.
    ///    + `Err(AppError)`: If the value is invalid.
    fn verify_string(&self, val: &str) -> Result<(), AppError> {
        if val.is_empty() {
            let err = self.invalid_val(val, None);
            return Err(err);
        }

        Ok(())
    }

    /// ## Verifies the u16 value.
    ///
    /// Function checks if the string value can be parsed
    /// into a u16 value.
    ///
    /// ## Arguments
    /// - `val`: `&str` - Value to verify.
    ///
    /// ## Returns
    /// - `Result<(), AppError>`:
    ///   + `Ok(())`: If the value is valid.
    ///   + `Err(AppError)`: If the value is invalid.
    fn verify_u16(&self, val: &str) -> Result<(), AppError> {
        match val.parse::<u16>() {
            Ok(_) => Ok(()),
            Err(e) => {
                let source = Some(Box::new(e) as Box<dyn std::error::Error>);
                let err = self.invalid_val(val, source);
                Err(err)
            }
        }
    }

    /// ## Verifies the enum value.
    ///
    /// Function checks if the value is in the list of allowed values.
    ///
    /// ## Arguments
    /// - `allowed_values`: `&[&str]` - List of allowed values.
    /// - `val`: `&str` - Value to verify.
    ///
    /// ## Returns
    /// - `Result<(), AppError>`:
    ///   + `Ok(())`: If the value is valid.
    ///   + `Err(AppError)`: If the value is invalid.
    fn verify_enum(&self, allowed_values: &[&str], val: &str) -> Result<(), AppError> {
        if allowed_values.contains(&val) {
            Ok(())
        } else {
            let err = self.invalid_val(val, None);
            Err(err)
        }
    }

    /// ## Verifies the file path.
    ///
    /// Function checks if the file path exists,
    /// it is a file and it is readable.
    ///
    /// ## Arguments
    /// - `val`: `&str` - File path to verify.
    ///
    /// ## Returns
    /// - `Result<(), AppError>`:
    ///   + `Ok(())`: If the file path is valid.
    ///   + `Err(AppError)`: If the file path is invalid.
    fn verify_file_path(&self, val: &str) -> Result<(), AppError> {
        let path = Path::new(val);

        if val.is_empty() || !path.exists() || !path.is_file() {
            let err = self.invalid_val(val, None);
            return Err(err);
        }

        // Check if the file is readable
        if let Err(e) = fs::File::open(path) {
            let source = Some(Box::new(e) as Box<dyn std::error::Error>);
            let err = self.invalid_val(val, source);
            return Err(err);
        }

        Ok(())
    }

    /// ## Constructs an error for the invalid value.
    ///
    /// Function constructs an error for the specified
    /// value.
    ///
    /// ## Arguments
    /// - `val`: `&str` - Invalid value.
    /// - `source`: `Option<Box<dyn std::error::Error>` - Source of the error.
    ///
    /// ## Returns
    /// - `AppError`: Error instance.
    fn invalid_val(&self, val: &str, source: Option<Box<dyn std::error::Error>>) -> AppError {
        let kind = ErrorKind::InvalidValueType;
        let message = self.construct_err_msg(val);

        AppError::new(kind, message, source)
    }

    /// Constructs an error message.
    ///
    /// Function constructs an error message for the invalid value.
    ///
    /// # Arguments
    /// - `val`: `&str` - Invalid value.
    ///
    /// # Returns
    /// - `String`: Error message.
    fn construct_err_msg(&self, val: &str) -> String {
        format!("{} {:?}: \"{}\"", INVALID_VALUE_FOR_TYPE, self, val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;

    // Test checks if the function can verify a valid string value.
    #[test]
    fn test_verify_string_valid() {
        let val: &str = "abc";
        let result: Result<(), AppError> = AppType::String.verify(val);
        assert_eq!(result, Ok(()));
    }

    // Test checks if the function can verify a string with numeric value.
    #[test]
    fn test_verify_string_numeric() {
        let val: &str = "123";
        let result: Result<(), AppError> = AppType::String.verify(val);

        assert_eq!(result, Ok(()));
    }

    // Test checks if the function returns an error when the string value is empty.
    #[test]
    fn test_verify_string_empty() {
        let val: &str = "";
        let result: Result<(), AppError> = AppType::String.verify(val);

        assert!(result.is_err());
    }

    // Test checks if the function can verify a valid u16 value.
    #[test]
    fn test_verify_u16_valid() {
        let val: &str = "65535";
        let result: Result<(), AppError> = AppType::U16.verify(val);

        assert_eq!(result, Ok(()));
    }

    // Test checks if the function returns an error when the value is not a u16.
    #[test]
    fn test_verify_u16_invalid() {
        let val: &str = "abc";
        let result: Result<(), AppError> = AppType::U16.verify(val);

        let sorurce_err = val.parse::<u16>().unwrap_err();
        let expected = AppType::U16.invalid_val(val, Some(Box::new(sorurce_err)));

        assert_eq!(result, Err(expected));
    }

    // Test checks if the function returns an error when the value is out of range (positive).
    #[test]
    fn test_verify_u16_out_of_range_positive() {
        let val: &str = "65536";
        let result: Result<(), AppError> = AppType::U16.verify(val);

        let sorurce_err = val.parse::<u16>().unwrap_err();
        let expected = AppType::U16.invalid_val(val, Some(Box::new(sorurce_err)));

        assert_eq!(result, Err(expected));
    }

    // Test checks if the function returns an error when the value is out of range (negative).
    #[test]
    fn test_verify_u16_out_of_range_negative() {
        let val: &str = "-1";
        let result: Result<(), AppError> = AppType::U16.verify(val);

        let sorurce_err = val.parse::<u16>().unwrap_err();
        let expected = AppType::U16.invalid_val(val, Some(Box::new(sorurce_err)));

        assert_eq!(result, Err(expected));
    }

    // Test checks if the function returns an error when the value is empty.
    #[test]
    fn test_verify_u16_empty() {
        let val: &str = "";
        let result: Result<(), AppError> = AppType::U16.verify(val);

        let sorurce_err = val.parse::<u16>().unwrap_err();
        let expected = AppType::U16.invalid_val(val, Some(Box::new(sorurce_err)));

        assert_eq!(result, Err(expected));
    }

    // Test checks if the function returns an error when the value is floating point.
    #[test]
    fn test_verify_u16_float() {
        let val: &str = "12.3";
        let result: Result<(), AppError> = AppType::U16.verify(val);

        let sorurce_err = val.parse::<u16>().unwrap_err();
        let expected = AppType::U16.invalid_val(val, Some(Box::new(sorurce_err)));

        assert_eq!(result, Err(expected));
    }

    // Test checks if the function can verify a valid enum value.
    #[test]
    fn test_verify_enum_valid() {
        let val: &str = "development";
        let allowed_values: &[&str] = &["development", "production"];
        let result: Result<(), AppError> = AppType::Enum(allowed_values).verify(val);

        assert_eq!(result, Ok(()));
    }

    // Test checks if the function returns an error when the value is not in the list of allowed values.
    #[test]
    fn test_verify_enum_invalid() {
        let val: &str = "staging";
        let allowed_values: &[&str] = &["development", "production"];
        let result: Result<(), AppError> = AppType::Enum(allowed_values).verify(val);

        let expected = AppType::Enum(allowed_values).invalid_val(val, None);

        assert_eq!(result, Err(expected));
    }

    // Test checks if the function can verify a valid file path.
    #[test]
    fn test_verify_file_path_valid() {
        let tmp_file = tempfile::NamedTempFile::new().unwrap();
        let val: &str = tmp_file.path().to_str().unwrap();

        let result: Result<(), AppError> = AppType::FilePath.verify(val);

        assert_eq!(result, Ok(()));
    }

    // Test checks if the function returns an error when the file path is empty.
    #[test]
    fn test_verify_file_path_empty() {
        let val: &str = "";
        let result: Result<(), AppError> = AppType::FilePath.verify(val);

        let expected = AppType::FilePath.invalid_val(val, None);

        assert_eq!(result, Err(expected));
    }

    // Test checks if the function returns an error when the file path does not exist.
    #[test]
    fn test_verify_file_path_not_exist() {
        let val: &str = "/path/to/file/that/does/not/exist";
        let result: Result<(), AppError> = AppType::FilePath.verify(val);

        let expected = AppType::FilePath.invalid_val(val, None);

        assert_eq!(result, Err(expected));
    }

    // Test checks if the function returns an error when the file path is a directory.
    #[test]
    fn test_verify_file_path_is_dir() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let val: &str = tmp_dir.path().to_str().unwrap();
        let result: Result<(), AppError> = AppType::FilePath.verify(val);

        let expected = AppType::FilePath.invalid_val(val, None);

        assert_eq!(result, Err(expected));
    }

    // Test checks if the function returns an error when the file path is not readable.
    #[test]
    fn test_verify_file_path_not_readable() {
        // !! This test is only for Unix systems, code will not run on Windows
        #[cfg(unix)]
        {
            let tmp_file = tempfile::NamedTempFile::new().unwrap();
            let val: &str = tmp_file.path().to_str().unwrap();

            // Change the file permissions to make it unreadable
            let mut perms = fs::metadata(val).unwrap().permissions();
            perms.set_mode(0o000); // Remove all permissions
            fs::set_permissions(val, perms).unwrap();

            let result: Result<(), AppError> = AppType::FilePath.verify(val);

            let source_err = fs::File::open(val).unwrap_err();
            let expected = AppType::FilePath.invalid_val(val, Some(Box::new(source_err)));

            assert_eq!(result, Err(expected));
        }
    }

    // Test how function constructs an error for the invalid value.
    #[test]
    fn test_construct_err() {
        let val: &str = "";
        let result: AppError = AppType::String.invalid_val(val, None);

        let expected_message = AppType::String.construct_err_msg(val);
        let expected = AppError::new(ErrorKind::InvalidValueType, expected_message, None);

        assert_eq!(result, expected);
    }

    // Test how function constructs an error message for the invalid value.
    #[test]
    fn test_construct_err_msg() {
        let val: &str = "";
        let result: String = AppType::String.construct_err_msg(val);
        let expected: String = format!(
            "{} {:?}: \"{}\"",
            INVALID_VALUE_FOR_TYPE,
            AppType::String,
            val
        );

        assert_eq!(result, expected);
    }
}
