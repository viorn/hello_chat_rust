use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::db::DbController;

use super::errors::ErrorResponse;

pub async fn create_user(
    Json(payload): Json<CreateUserRequest>,
    Extension(db_controller): Extension<Arc<DbController>>,
) -> impl IntoResponse {
    //check password
    let d_pass_regexp = Regex::new(r".*(.*\d).*").unwrap();
    let w_pass_regexp = Regex::new(r".*([a-zA-Z]).*").unwrap();
    if payload.password.len() < 6
        || !d_pass_regexp.is_match(&payload.password)
        || !w_pass_regexp.is_match(&payload.password)
    {
        return (
            StatusCode::BAD_REQUEST,
            Json(CreateUserResponse::Error {
                body: ErrorResponse::wrong_password_secure(),
            }),
        );
    }
    //
    let pass = sha256::digest(payload.password);
    let id_result = db_controller
        .create_user(payload.name.clone(), payload.email.clone(), pass)
        .await;
    match id_result {
        Ok(id) => (
            StatusCode::CREATED,
            Json(CreateUserResponse::Result {
                id: id,
                name: payload.name,
                email: payload.email,
            }),
        ),
        Err(error) => match error {
            crate::db::user::Error::NameIsOccupied => (
                StatusCode::BAD_REQUEST,
                Json(CreateUserResponse::Error {
                    body: ErrorResponse::field_is_occupied("name"),
                }),
            ),
            crate::db::user::Error::EmailIsOccupied => (
                StatusCode::BAD_REQUEST,
                Json(CreateUserResponse::Error {
                    body: ErrorResponse::field_is_occupied("email"),
                }),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(CreateUserResponse::Error {
                    body: ErrorResponse::unknown(),
                }),
            ),
        },
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserRequest {
    name: String,
    email: String,
    password: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
enum CreateUserResponse {
    #[serde(rename_all = "camelCase")]
    Result {
        id: i32,
        name: String,
        email: String,
    },
    #[serde(rename_all = "camelCase")]
    Error {
        #[serde(flatten)]
        body: ErrorResponse,
    },
}

impl ErrorResponse {
    fn wrong_password_secure() -> Self {
        Self {
            name: "wrong_password_secure".to_string(),
            message:
                "the password must consist of letters and numbers, and have at least 6 characters"
                    .to_string(),
        }
    }
}
