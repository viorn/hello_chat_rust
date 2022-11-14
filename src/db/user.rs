use sqlx::Row;


use super::{data, sql::user, DbController};

pub enum Error {
    Unknown,
    Sql { message: String },
    NameIsOccupied,
    EmailIsOccupied,
    NotFound,
}

impl DbController {
    pub async fn create_user(
        &self,
        name: String,
        email: String,
        password: String,
    ) -> Result<i32, Error> {
        let res = sqlx::query(&user::insert(name, email, password))
            .fetch_one(&self.pool)
            .await;
        match res {
            Ok(row) => Ok(row.get(0)),
            Err(error) => match error {
                sqlx::Error::Database(db_error) => {
                    let message = db_error.as_ref().message();
                    if message.contains("unique constraint") {
                        if message.contains("name_key") {
                            return Err(Error::NameIsOccupied);
                        }
                        if message.contains("email_key") {
                            return Err(Error::EmailIsOccupied);
                        }
                    }
                    Err(Error::Sql {
                        message: message.to_string(),
                    })
                }
                _ => Err(Error::Unknown),
            },
        }
    }

    /*pub async fn get_user_by_id(&self, id: i32) -> Result<data::User, Error> {
        match sqlx::query(&user::select_by_id(id))
            .fetch_one(&self.pool)
            .await
        {
            Ok(res) => Ok(data::User {
                id: res.get(user::ID),
                name: res.get(user::NAME),
                email: res.get(user::EMAIL),
                password: res.get(user::PASSWORD),
            }),
            Err(error) => match error {
                _ => Err(Error::NotFound),
            },
        }
    }*/

    pub async fn get_user_by_name(&self, name: &String) -> Result<data::User, Error> {
        match sqlx::query(&user::select_by_name(name))
            .fetch_one(&self.pool)
            .await
        {
            Ok(res) => Ok(data::User {
                id: res.get(user::ID),
                name: res.get(user::NAME),
                email: res.get(user::EMAIL),
                password: res.get(user::PASSWORD),
            }),
            Err(error) => match error {
                _ => Err(Error::NotFound),
            },
        }
    }
}
