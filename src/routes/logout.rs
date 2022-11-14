use std::sync::Arc;

use axum::{
    http::{StatusCode, HeaderMap},
    response::IntoResponse,
    Json, Extension
};
use serde::Serialize;
use crate::db::DbController;

use super::jwt::check_token;
use super::errors::ErrorResponse;


pub async fn logout(
    headers: HeaderMap,
    Extension(db_controller): Extension<Arc<DbController>>,
) -> impl IntoResponse {
    match check_token(&headers) {
        Ok(old_token) => {
            match db_controller.delete_token(&old_token.0).await {
                Ok(_) => return (StatusCode::OK, Json(RefreshTokenResponse::Result {  })),
                Err(_) => return (StatusCode::UNAUTHORIZED, Json(RefreshTokenResponse::Error { body: ErrorResponse::unknown() })),
            };
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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
enum RefreshTokenResponse {
    #[serde(rename_all = "camelCase")]
    Result {
    },
    #[serde(rename_all = "camelCase")]
    Error {
        #[serde(flatten)]
        body: ErrorResponse
    }
}