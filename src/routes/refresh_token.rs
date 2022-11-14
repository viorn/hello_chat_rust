use std::sync::Arc;

use axum::{
    http::{StatusCode, HeaderMap},
    response::IntoResponse,
    Json, Extension
};
use serde::{Deserialize, Serialize};
use crate::db::DbController;

use super::jwt::{check_token, encode_token, Claims, random_string, get_ext_time};
use super::errors::{ErrorResponse};


pub async fn refresh_token(
    Json(payload): Json<RefreshTokenRequest>,
    headers: HeaderMap,
    Extension(db_controller): Extension<Arc<DbController>>,
) -> impl IntoResponse {
    match check_token(&headers) {
        Ok(old_token) => {
            let refersh_token = match db_controller.select_token(&old_token.0).await {
                Ok(res) => res,
                Err(_) => return (StatusCode::UNAUTHORIZED, Json(RefreshTokenResponse::Error { body: ErrorResponse::unknown() })),
            };
            if refersh_token.refresh_token.eq(&payload.refresh_token) {
                match encode_token(Claims {
                    salt1: random_string(32),
                    user_id: old_token.1.user_id.clone(),
                    exp: get_ext_time() as usize,
                    cuuid: old_token.1.cuuid.clone(),
                    salt2: random_string(32)
                }) {
                    Ok(new_token) => {
                        let new_refresh = random_string(128);
                        match db_controller.delete_token(&old_token.0).await {
                            Ok(_) => match db_controller.add_token(&new_token, &new_refresh, &old_token.1.user_id, &old_token.1.cuuid).await {
                                Ok(_) => {},
                                Err(_) => return (StatusCode::UNAUTHORIZED, Json(RefreshTokenResponse::Error { body: ErrorResponse::unknown() })),
                            },
                            Err(_) => return (StatusCode::UNAUTHORIZED, Json(RefreshTokenResponse::Error { body: ErrorResponse::unknown() })),
                        };
                        (StatusCode::OK, Json(RefreshTokenResponse::Result {
                            user_id: old_token.1.user_id,
                            token: new_token,
                            refresh_token: new_refresh
                        }))
                    },
                    Err(_) => todo!(),
                }
            } else {
                (StatusCode::UNAUTHORIZED, Json(RefreshTokenResponse::Error { body: ErrorResponse::unknown() }))
            }
        },
        Err(error) => match error {
            super::jwt::Error::TokenIsExpire => (StatusCode::UNAUTHORIZED, Json(RefreshTokenResponse::Error {
                body: ErrorResponse::token_is_expire()
            })),
            super::jwt::Error::Unknown => (StatusCode::UNAUTHORIZED, Json(RefreshTokenResponse::Error {
                body: ErrorResponse::unknown()
            })),
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshTokenRequest {
    refresh_token: String
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
enum RefreshTokenResponse {
    #[serde(rename_all = "camelCase")]
    Result {
        user_id: i32,
        token: String,
        refresh_token: String,
        
    },
    #[serde(rename_all = "camelCase")]
    Error {
        #[serde(flatten)]
        body: ErrorResponse
    }
}