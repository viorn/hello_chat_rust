use sqlx::Row;

use super::{DbController, sql::token, data};

pub enum Error {
    Sql{sql_error: sqlx::Error}
}

impl DbController {
    pub async fn add_token(&self, token: &String, refresh: &String, user_id: &i32, cuuid: &String) -> Result<(), Error> {
        match sqlx::query(&token::insert(token, refresh, user_id, cuuid)).execute(&self.pool).await {
            Ok(_) => Ok(()),
            Err(sql_err) => match sql_err {
                raw => Err(Error::Sql { sql_error: raw }),
            },
        } 
    }

    pub async fn select_token(&self, token: &String) -> Result<data::RefreshToken, Error> {
        match sqlx::query(&token::select_by_token(token)).fetch_one(&self.pool).await {
            Ok(res) => Ok(data::RefreshToken {
                token: res.get(token::TOKEN),
                refresh_token: res.get(token::REFRESH),
                user_id: res.get(token::USER_ID),
                cuuid: res.get(token::CUUID)
            }),
            Err(sql_err) => match sql_err {
                raw => Err(Error::Sql { sql_error: raw }),
            },
        } 
    }

    pub async fn delete_token_by_cuuid(&self, cuuid: &String) -> Result<(), Error> {
        match sqlx::query(&token::delete_by_cuuid(cuuid)).execute(&self.pool).await {
            Ok(_) => Ok(()),
            Err(sql_err) => match sql_err {
                raw => Err(Error::Sql { sql_error: raw }),
            },
        } 
    }

    pub async fn delete_token(&self, token: &String) -> Result<(), Error> {
        match sqlx::query(&token::delete_by_token(token)).execute(&self.pool).await {
            Ok(_) => Ok(()),
            Err(sql_err) => match sql_err {
                raw => Err(Error::Sql { sql_error: raw }),
            },
        } 
    }
}