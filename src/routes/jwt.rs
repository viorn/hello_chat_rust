use std::time::{UNIX_EPOCH, SystemTime};

use axum::http::HeaderMap;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};

pub fn check_token(headers: &HeaderMap) -> Result<(String, Claims), Error> {
    match headers.get("token") {
        Some(token) => match token.to_str() {
            Ok(token) => decode_token(token.to_string()),
            Err(_) => Result::Err(Error::Unknown),
        },
        None => Result::Err(Error::Unknown),
    }
}

pub fn get_ext_time() -> u64{
    SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                + 30
}

pub enum Error {
    TokenIsExpire,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub salt1: String,
    pub user_id: i32,
    pub exp: usize,
    pub cuuid: String,
    pub salt2: String,
}
pub fn encode_token(claims: Claims) -> Result<String, Error> {
    match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("secret".as_ref()),
    ) {
        Ok(res) => Ok(res),
        Err(_) => Err(Error::Unknown),
    }
}

pub fn decode_token(token: String) -> Result<(String, Claims), Error> {
    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default(),
    ) {
        Ok(res) => Ok((token, res.claims)),
        Err(er) => match er.into_kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => Err(Error::TokenIsExpire),
            _ => Err(Error::Unknown),
        },
    }
}

pub fn random_string(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}
