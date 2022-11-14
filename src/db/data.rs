pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String
}

pub struct RefreshToken {
    pub token: String,
    pub refresh_token: String,
    pub cuuid: String,
    pub user_id: i32,
}