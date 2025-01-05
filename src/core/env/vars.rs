use once_cell::sync::Lazy;
use std::{collections::HashSet, error};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

// Local imports
use crate::{
    core::{
        config::APP_CONFIG,
        err::{AppError, ErrorKind},
        types::AppType,
    },
    // prelude::is_u16,
    strings::{
        env::vars::{
            DB_HOST, DB_NAME, DB_PASS, DB_PORT, DB_SSL_MODE, DB_USER, PATH_TO_DB_SSL_ROOT_CERT,
        },
        postgres::{
            ALLOW_SSL, DISABLE_SSL, PREFER_SSL, REQUIRE_SSL, VERIFY_CA_SSL, VERIFY_FULL_SSL,
        },
    },
};

static APP_PREFIX: Lazy<&str> = Lazy::new(|| {
    APP_CONFIG
        .as_ref()
        .expect("Failed get app configuration")
        .app
        .prefix
        .as_str()
});

// * Environment variables to validate
// * keep it up to date with the .env.example,
// * .env files and
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum RequiredEnvVar {
    // Test, // !! delete
    DbName,
    DbHost,
    DbPort,
    DbUser,
    DbPass,
    DbSslMode,
    PathToDbSslRootCert,
}

impl EnvVar for RequiredEnvVar {
    type VarType = Self;

    fn all() -> HashSet<Self> {
        Self::iter().collect()
    }

    fn name(&self) -> String {
        match self {
            // Self::Test => construct_name(*APP_PREFIX, "TEST"), // !! delete
            Self::DbName => construct_name(*APP_PREFIX, DB_NAME),
            Self::DbHost => construct_name(*APP_PREFIX, DB_HOST),
            Self::DbPort => construct_name(*APP_PREFIX, DB_PORT),
            Self::DbUser => construct_name(*APP_PREFIX, DB_USER),
            Self::DbPass => construct_name(*APP_PREFIX, DB_PASS),
            Self::DbSslMode => construct_name(*APP_PREFIX, DB_SSL_MODE),
            Self::PathToDbSslRootCert => construct_name(*APP_PREFIX, PATH_TO_DB_SSL_ROOT_CERT),
        }
    }

    fn value(&self) -> String {
        std::env::var(self.name()).expect("Failed to get env var value")
    }

    fn type_(&self) -> AppType {
        match self {
            // Self::Test => AppType::String, // !! delete
            Self::DbName => AppType::String,
            Self::DbHost => AppType::String,
            Self::DbPort => AppType::U16,
            Self::DbUser => AppType::String,
            Self::DbPass => AppType::String,
            Self::DbSslMode => AppType::Enum(&[
                DISABLE_SSL,
                ALLOW_SSL,
                PREFER_SSL,
                REQUIRE_SSL,
                VERIFY_CA_SSL,
                VERIFY_FULL_SSL,
            ]),
            Self::PathToDbSslRootCert => AppType::FilePath,
        }
    }

    fn verify(&self) -> Result<(), AppError> {
        self.type_().verify(self.value().as_str())
    }

    fn verify_all() -> Result<(), AppError> {
        let vars: HashSet<Self> = Self::all();

        for var in vars {
            var.verify()?;
        }

        Ok(())
    }
}

pub trait EnvVar {
    type VarType; // Associated type for the type implementing the trait

    fn all() -> HashSet<Self::VarType>
    where
        Self: Sized;

    fn name(&self) -> String;

    fn value(&self) -> String;

    fn type_(&self) -> AppType;

    fn verify(&self) -> Result<(), AppError>;

    fn verify_all() -> Result<(), AppError>;
}

fn construct_name(prefix: &str, name: &str) -> String {
    format!("{}{}", prefix, name)
}
