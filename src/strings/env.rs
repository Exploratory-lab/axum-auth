pub mod msgs {}

pub mod vars {
    //! Constants for environment variables,
    //! their names and example values.
    //!
    //! Use these constants to access environment variables
    //! in the application.

    // Database name to open pool connection to
    pub const DB_NAME: &str = "DB_NAME";

    // Database host address
    pub const DB_HOST: &str = "DB_HOST";

    // Database connection port
    pub const DB_PORT: &str = "DB_PORT";

    // Database user
    pub const DB_USER: &str = "DB_USER";

    // Database password
    pub const DB_PASS: &str = "DB_PASS";

    // Database ssl mode
    pub const DB_SSL_MODE: &str = "DB_SSL_MODE";

    // Database ssl root certificate
    pub const PATH_TO_DB_SSL_ROOT_CERT: &str = "PATH_TO_DB_SSL_ROOT_CERT";
}
