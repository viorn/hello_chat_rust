mod sql;
pub mod data;
pub mod user;
pub mod token;



use std::fmt;

use sqlx::{Pool, postgres::{PgPoolOptions}};
use sqlx::Postgres;

use self::sql::*;

pub enum Error {
    ConnectError,
    DbCreateError
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ConnectError => f.write_str("ConnectError"),
            Error::DbCreateError => f.write_str("DbCreateError")
        }
    }
}

pub async fn create_pool() -> Result<DbController, Error> {
    let db = match PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://postgres:qwerty123@localhost:5432/postgres")
            .await {
        Ok(pool) => DbController {
            pool: pool
        },
        Err(_) => return Err(Error::ConnectError),
    };
    match db.init().await {
        Ok(_) => {},
        Err(error) => return Err(error),
    };
    Ok(db)
}

pub struct DbController {
    pool: Pool<Postgres>
}

impl DbController {
    async fn init(&self) -> Result<(), Error> {
        match sqlx::query(&create_shema()).execute(&self.pool).await {
            Ok(_) => {},
            Err(_) => return Err(Error::DbCreateError),
        };
        match sqlx::query(&sql::user::create_table()).execute(&self.pool).await {
            Ok(_) => {},
            Err(_) => return Err(Error::DbCreateError),
        };
        match sqlx::query(&sql::token::create_table()).execute(&self.pool).await {
            Ok(_) => {},
            Err(_) => return Err(Error::DbCreateError),
        };
        Ok(())
    }
}