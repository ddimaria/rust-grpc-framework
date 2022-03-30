//! Custom errors for this applicatoin.
//!
//! Map errors from libraries to Error.
//!
//! Define a reusable Result type.

use argon2::Error as Argon2Error;
use envy::Error as EnvyError;
use log::error;
use std::net::AddrParseError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}.  Make sure you copied .env.example to .env")]
    Config(String),

    #[error("Unable to decode the JWT: {0}")]
    DecodeJwtToken(String),

    #[error("Unable to encode the JWT: {0}")]
    EncodeJwtToken(String),

    #[error("Hash error: {0}.")]
    Hash(String),

    #[error("Parse error: {0}.")]
    Parse(String),

    #[error("SQLx database pool error: {0}.")]
    DatabasePool(String),

    #[error("Unknown error: {0}.")]
    Unknown(String),
}

// Log out errors
fn log_error(error: Error) -> Error {
    error!("{:?}", error);
    error
}

impl From<EnvyError> for Error {
    fn from(error: EnvyError) -> Self {
        let error = match error {
            EnvyError::MissingValue(error) => format!("Missing config value in .env: {}", error),
            EnvyError::Custom(error) => error,
        };
        log_error(Error::Config(error))
    }
}

impl From<AddrParseError> for Error {
    fn from(error: AddrParseError) -> Self {
        log_error(Error::Parse(error.to_string()))
    }
}

impl From<Argon2Error> for Error {
    fn from(error: Argon2Error) -> Self {
        log_error(Error::Hash(error.to_string()))
    }
}
