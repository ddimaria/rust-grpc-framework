//! Inject dotenv and env variables into the Config struct
//!
//! The envy crate injects environment variables into a struct.
//!
//! dotenv allows environment variables to be augmented/overwriten by a
//! .env file.
//!
//! This file throws the Config struct into a CONFIG lazy_static to avoid
//! multiple processing.

use crate::error::Result;
use dotenv::dotenv;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub auth_salt: String,
    pub jwt_expiration: i64,
    pub jwt_key: String,
    // pub redis_url: String,
    // pub rust_backtrace: u8,
    // pub rust_log: String,
    // pub server: String,
    // pub session_key: String,
    // pub session_name: String,
    // pub session_secure: bool,
    // pub session_timeout: i64,
    pub(crate) server: String,
    pub(crate) database_url: String,
}

// Throw the Config struct into a CONFIG lazy_static to avoid multiple processing
lazy_static! {
    pub static ref CONFIG: Config = config().unwrap_or_else(|error| panic!("{}", error));
}

/// Use envy to inject dotenv and env vars into the Config struct
pub fn config() -> Result<Config> {
    dotenv().ok();
    Ok(envy::from_env::<Config>()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_gets_the_config() {
        assert_ne!(config().unwrap().server, "".to_string());
        assert_ne!(*CONFIG.server, "".to_string());
    }
}
