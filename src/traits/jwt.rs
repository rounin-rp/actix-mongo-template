use crate::{
    handlers::error_handler::{Errors, HttpErrors},
    helpers::enums,
};
use actix_web::{http, FromRequest};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};

impl Default for enums::JwtTokenType {
    fn default() -> Self {
        Self::Access
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct JwtToken {
    pub user_id: String,
    pub token_type: enums::JwtTokenType,
    pub expiry: u64,
}

impl JwtToken {
    pub fn create_fresh_pair(user_id: impl Into<String>) -> Result<(String, String), Errors> {
        let access_token = Self {
            user_id: user_id.into().clone(),
            token_type: enums::JwtTokenType::Refresh,
            expiry: (Utc::now() + Duration::days(1)).timestamp() as u64,
        };
        Ok((access_token.encode_self()?, access_token.encode_self()?))
    }
    fn encode_self(&self) -> Result<String, Errors> {
        let secret = String::default();
        encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .map_err(|error| Errors::InternalError(error.to_string()))
    }
    pub fn decode(token: String) -> Result<Self, Errors> {
        let secret = String::default();
        let token_data_result = decode::<Self>(
            token.as_str(),
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        );
        match token_data_result {
            Ok(token_data) => {
                let jwt_token = token_data.claims;
                if jwt_token.has_expired() {
                    Err(Errors::HttpError(HttpErrors::Unauthorized))
                } else {
                    Ok(jwt_token)
                }
            }
            Err(_) => Err(Errors::HttpError(HttpErrors::BadRequest)),
        }
    }
    pub fn has_expired(&self) -> bool {
        self.expiry < Utc::now().timestamp() as u64
    }
}
impl FromRequest for JwtToken {
    type Error = Errors;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let header = req.headers().get(http::header::AUTHORIZATION);
        let token = match header {
            Some(token_value) => token_value.to_str().unwrap(),
            None => "",
        };
        let token = match token.strip_prefix("Bearer ") {
            Some(token) => token.to_string(),
            None => return ready(Err(Errors::HttpError(HttpErrors::Unauthorized))),
        };

        let decoded_token_result = match Self::decode(token) {
            Ok(decoded_token) => decoded_token,
            Err(_) => return ready(Err(Errors::HttpError(HttpErrors::Unauthorized))),
        };
        /*
        let client = match req.app_data::<web::Data<MongoClient>>() {
            Some(client) => client.get_ref().clone(),
            None => return ready(Err(Errors::HttpError(HttpErrors::BadRequest))),
        };
        */

        ready(Ok(decoded_token_result))
    }
}
