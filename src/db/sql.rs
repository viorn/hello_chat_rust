pub fn create_shema() -> String {
    "CREATE SCHEMA IF NOT EXISTS APP".to_string()
}

pub mod user {
    pub const TABLE: &str = "app.users";
    pub const ID: &str = "id";
    pub const NAME: &str = "name";
    pub const EMAIL: &str = "email";
    pub const PASSWORD: &str = "password";

    pub fn create_table() -> String {
        format!(
            r#"
        CREATE TABLE IF NOT EXISTS {} (
            {} SERIAL PRIMARY KEY,
            {} varchar(500) UNIQUE,
            {} varchar(500)     ,
            {} varchar(500)
          )
        "#,
            TABLE, ID, NAME, EMAIL, PASSWORD
        )
    }

    pub fn insert(name: String, email: String, password: String) -> String {
        format!(
            r#"
        INSERT INTO {} ({}, {}, {})
              VALUES('{name}', '{email}', '{password}')
              RETURNING ID
        "#,
            TABLE,
            NAME,
            EMAIL,
            PASSWORD,
            name = name,
            email = email,
            password = password
        )
    }

    /*pub fn select_by_id(id: i32) -> String {
        format!("SELECT * FROM {} WHERE {}={id}", TABLE, ID, id = id)
    }*/

    pub fn select_by_name(name: &str) -> String {
        format!(
            "SELECT * FROM {} WHERE {}='{name}'",
            TABLE,
            NAME,
            name = name
        )
    }
}

pub mod token {

    pub const TABLE: &str = "app.tokens";
    pub const TOKEN: &str = "token";
    pub const REFRESH: &str = "refresh";
    pub const USER_ID: &str = "user_id";
    pub const CUUID: &str = "cuuid";
    pub const CREATE_DATE: &str = "create_date";

    pub fn create_table() -> String {
        format!(
            r#"
        CREATE TABLE IF NOT EXISTS {} (
            {} VARCHAR(512) PRIMARY KEY,
            {} VARCHAR(128),
            {} SERIAL,
            {} VARCHAR(128),
            {} TIMESTAMP default CURRENT_TIMESTAMP not null
          )
        "#,
            TABLE, TOKEN, REFRESH, USER_ID, CUUID, CREATE_DATE
        )
    }

    pub fn insert(token: &String, refresh: &String, user_id: &i32, cuuid: &String) -> String {
        format!(r#"
        INSERT INTO {} ({}, {}, {}, {})
            VALUES('{token}', '{refresh}', '{cuuid}' , {user_id})
        "#, TABLE, TOKEN, REFRESH, CUUID, USER_ID, token = token, refresh = refresh, user_id = user_id, cuuid = cuuid)
    }

    pub fn select_by_token(token: &String) -> String {
        format!(r#"
        SELECT * FROM {} WHERE {}='{}'
        "#, TABLE, TOKEN, token)
    }

    pub fn delete_by_token(token: &String) -> String {
        format!(r#"
        DELETE FROM {} WHERE {}='{}'
        "#, TABLE, TOKEN, token)
    }

    pub fn delete_by_cuuid(cuuid: &String) -> String {
        format!(r#"
        DELETE FROM {} WHERE {}='{}'
        "#, TABLE, CUUID, cuuid)
    }
}
