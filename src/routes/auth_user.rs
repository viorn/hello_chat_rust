use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Extension, Json};

use serde::{Deserialize, Serialize};

use crate::db::DbController;

use super::{
    errors::ErrorResponse,
    jwt::{encode_token, random_string, Claims, get_ext_time},
};

pub async fn auth_user(
    Json(payload): Json<AuthRequest>,
    Extension(db_controller): Extension<Arc<DbController>>,
) -> impl IntoResponse {
    match db_controller.get_user_by_name(&payload.name).await {
        Ok(user) => {
            if !user.password.eq(&sha256::digest(payload.password)) {
                return (
                    StatusCode::UNAUTHORIZED,
                    Json(AuthResponse::Error {
                        body: ErrorResponse::wrong_password(),
                    }),
                );
            }

            let time = get_ext_time();
            let refresh_token = random_string(128);
            let claims = Claims {
                salt1: random_string(32),
                user_id: user.id,
                exp: time as usize,
                cuuid: payload.cuuid.clone(),
                salt2: random_string(32),
            };

            // Create the authorization token
            match encode_token(claims) {
                Ok(token) => {
                    match db_controller.delete_token_by_cuuid(&payload.cuuid).await {
                        Ok(_) => {},
                        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(AuthResponse::Error { body: ErrorResponse::unknown() })),
                    }
                    match db_controller
                        .add_token(&token, &refresh_token, &user.id, &payload.cuuid)
                        .await
                    {
                        Ok(_) => (
                            StatusCode::OK,
                            Json(AuthResponse::Result {
                                user_id: user.id,
                                token: token,
                                refresh_token: refresh_token,
                            }),
                        ),
                        Err(error) => match error {
                            crate::db::token::Error::Sql { sql_error } => (
                                StatusCode::UNAUTHORIZED,
                                Json(AuthResponse::Error {
                                    body: ErrorResponse {
                                        name: "sql error".to_string(),
                                        message: format!(
                                            "{}",
                                            sql_error.as_database_error().unwrap().message()
                                        ),
                                    },
                                }),
                            ),
                        },
                    }
                }
                Err(_) => (
                    StatusCode::UNAUTHORIZED,
                    Json(AuthResponse::Error {
                        body: ErrorResponse {
                            name: "encode_token".to_string(),
                            message: "encode token error".to_string(),
                        },
                    }),
                ),
            }
        }
        Err(error) => match error {
            _ => (
                StatusCode::UNAUTHORIZED,
                Json(AuthResponse::Error {
                    body: ErrorResponse::user_not_found(),
                }),
            ),
        },
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthRequest {
    name: String,
    password: String,
    cuuid: String
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
enum AuthResponse {
    #[serde(rename_all = "camelCase")]
    Result {
        user_id: i32,
        token: String,
        refresh_token: String,
    },
    #[serde(rename_all = "camelCase")]
    Error {
        #[serde(flatten)]
        body: ErrorResponse,
    },
}

impl ErrorResponse {
    pub fn user_not_found() -> Self {
        Self {
            name: "user_not_found".to_string(),
            message: "user not found".to_string(),
        }
    }
    pub fn wrong_password() -> Self {
        Self {
            name: "wrong_password".to_string(),
            message: "wrong password".to_string(),
        }
    }
}
