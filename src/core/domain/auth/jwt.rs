use crate::core::domain::error::{AppError, Result};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject (user ID)
    pub email: String,
    pub role: String,
    pub exp: usize, // Expiration time
    pub iat: usize, // Issued at
}

#[derive(Clone)]
pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    validation: Validation,
}

impl JwtService {
    pub fn new(secret: &str) -> Self {
        let encoding_key = EncodingKey::from_secret(secret.as_ref());
        let decoding_key = DecodingKey::from_secret(secret.as_ref());
        let validation = Validation::default();

        Self {
            encoding_key,
            decoding_key,
            validation,
        }
    }

    pub fn generate_token(&self, user_id: Uuid, email: &str, role: &str) -> Result<String> {
        let now = Utc::now();
        let expires_at = now + Duration::hours(24); // Token valid for 24 hours

        let claims = Claims {
            sub: user_id.to_string(),
            email: email.to_string(),
            role: role.to_string(),
            exp: expires_at.timestamp() as usize,
            iat: now.timestamp() as usize,
        };

        encode(&Header::default(), &claims, &self.encoding_key).map_err(|_| AppError::Internal)
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims> {
        decode::<Claims>(token, &self.decoding_key, &self.validation)
            .map(|data| data.claims)
            .map_err(|_| AppError::Authentication {
                message: "Invalid or expired token".to_string(),
            })
    }

    pub fn extract_user_id(&self, token: &str) -> Result<Uuid> {
        let claims = self.verify_token(token)?;
        Uuid::parse_str(&claims.sub).map_err(|_| AppError::Authentication {
            message: "Invalid user ID in token".to_string(),
        })
    }
}
