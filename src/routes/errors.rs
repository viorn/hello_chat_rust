use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub name: String,
    pub message: String
}

impl ErrorResponse {
    pub fn field_is_occupied(field: &str) -> Self {
        Self {
            name: format!("{}_is_occupied", field),
            message: format!("enter a different {}", field)
        }
    }
    pub fn token_is_expire() -> Self {
        Self {
            name: "token_is_expire".to_string(),
            message: "your token is expire".to_string()
        }
    }
    pub fn unknown() -> Self {
        Self {
            name: "unknown".to_string(),
            message: "unknown error".to_string()
        }
    }
}