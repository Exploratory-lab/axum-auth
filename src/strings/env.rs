pub mod msgs {}

pub mod vars {
    //! Constants for environment variables,
    //! their names and example values.
    //!
    //! Use these constants to access environment variables
    //! in the application.
    //!
    //! # Examples
    //!
    //! ```
    //! use axum_auth::strings::env::vars::{DB_NAME};
    //! use std::collections::HashMap;
    //!
    //! let vars = HashMap::from([(DB_NAME, "test_db")]);
    //!
    //! assert_eq!(vars.get(DB_NAME), Some(&"test_db"));
    //! ```

    // !! DELETE THIS LINE
    pub const TEST: &str = "TEST";

    // Database name to open pool connection to
    pub const DB_NAME: &str = "DB_NAME";
    pub const DB_NAME_EXAMPLE: &str = "test_db";

    // Database host address
    pub const DB_HOST: &str = "DB_HOST";
    pub const DB_HOST_EXAMPLE: &str = "localhost";

    // Database connection port
    pub const DB_PORT: &str = "DB_PORT";
    pub const DB_PORT_EXAMPLE: &str = "5432";

    // Database user
    pub const DB_USER: &str = "DB_USER";
    pub const DB_USER_EXAMPLE: &str = "test_user";

    // Database password
    pub const DB_PASS: &str = "DB_PASS";
    pub const DB_PASS_EXAMPLE: &str = "test_pass";

    // Database ssl mode
    pub const DB_SSL_MODE: &str = "DB_SSL_MODE";
    pub const DB_SSL_MODE_EXAMPLE: &str = "verify-full";

    // Database ssl root certificate
    pub const PATH_TO_DB_SSL_ROOT_CERT: &str = "PATH_TO_DB_SSL_ROOT_CERT";
    pub const PATH_TO_DB_SSL_ROOT_CERT_EXAMPLE: &str = "/path/to/root.crt";
}
