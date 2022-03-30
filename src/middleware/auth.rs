//! Handle JWTs, hash passwords, and Identity Service

use crate::config::CONFIG;
use crate::error::{Error, Result};
use argon2::{self, Config};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use tonic::metadata::MetadataValue;
use tonic::{Request, Status};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub(crate) struct PrivateClaim {
    pub(crate) user_id: Uuid,
    pub(crate) email: String,
    exp: i64,
}

impl PrivateClaim {
    pub(crate) fn new(user_id: Uuid, email: String) -> Self {
        Self {
            user_id,
            email,
            exp: (Utc::now() + Duration::hours(CONFIG.jwt_expiration)).timestamp(),
        }
    }
}

/// Create a json web token (JWT)
pub(crate) fn create_jwt(private_claim: &PrivateClaim) -> Result<String> {
    let encoding_key = EncodingKey::from_secret(&CONFIG.jwt_key.as_ref());
    encode(&Header::default(), &private_claim, &encoding_key)
        .map_err(|e| Error::EncodeJwtToken(e.to_string()))
}

/// Decode a json web token (JWT)
pub(crate) fn decode_jwt(token: &str) -> Result<PrivateClaim> {
    let decoding_key = DecodingKey::from_secret(&CONFIG.jwt_key.as_ref());
    decode::<PrivateClaim>(token, &decoding_key, &Validation::default())
        .map(|data| data.claims)
        .map_err(|e| Error::DecodeJwtToken(e.to_string()))
}

/// Encrypt a password
///
/// Uses the argon2i algorithm.
/// auth_salt is environment-configured.
pub(crate) fn hash(password: &str) -> Result<String> {
    let config = Config::default();
    let hashed = argon2::hash_encoded(password.as_bytes(), &CONFIG.auth_salt.as_bytes(), &config)?;

    Ok(hashed)
}

pub(crate) fn validate_token(req: Request<()>) -> std::result::Result<Request<()>, Status> {
    // let token = MetadataValue::from_str("Bearer some-secret-token").unwrap();
    let token = req.metadata().get("authorization");

    if let Some(token) = token {
        // TODO: remove unwraps
        let private_claim = decode_jwt(token.to_str().unwrap()).unwrap();
    }

    Err(Status::unauthenticated("No valid auth token"))
}

#[cfg(test)]
pub mod tests {
    use super::*;
    static EMAIL: &str = "test@test.com";

    #[test]
    fn it_hashes_a_password() {
        let password = "password";
        let hashed = hash(password).unwrap();
        assert_ne!(password, hashed);
    }

    #[test]
    fn it_matches_2_hashed_passwords() {
        let password = "password";
        let hashed = hash(password).unwrap();
        let hashed_again = hash(password).unwrap();
        assert_eq!(hashed, hashed_again);
    }

    #[test]
    fn it_creates_a_jwt() {
        let private_claim = PrivateClaim::new(Uuid::new_v4(), EMAIL.into());
        let jwt = create_jwt(&private_claim);
        assert!(jwt.is_ok());
    }

    #[test]
    fn it_decodes_a_jwt() {
        let private_claim = PrivateClaim::new(Uuid::new_v4(), EMAIL.into());
        let jwt = create_jwt(&private_claim).unwrap();
        let decoded = decode_jwt(&jwt).unwrap();
        assert_eq!(private_claim, decoded);
    }
}
