use crate::prelude::*;
mod sql;
use std::convert::{TryFrom, TryInto};
use tokio_postgres::Client;
#[rocket::async_trait]
impl DBConnection for Client {
    async fn init(&self) -> Result<()> {
        self.execute(sql::CREATE_TABLE, &[]).await?;
        Ok(())
    }
    async fn create_user(&self, username: &str, hash: &str, is_admin: bool) -> Result<(), Error> {
        self.execute(sql::INSERT_USER, &[&username, &hash, &is_admin])
            .await?;
        Ok(())
    }
    async fn update_user(&self, user: &User) -> Result<()> {
        self.execute(
            sql::UPDATE_USER,
            &[&user.username, &user.password, &user.is_admin],
        )
        .await?;
        Ok(())
    }
    async fn delete_user_by_id(&self, user_id: i32) -> Result<()> {
        self.execute(sql::REMOVE_BY_ID, &[&user_id]).await?;
        Ok(())
    }
    async fn delete_user_by_username(&self, username: &str) -> Result<()> {
        self.execute(sql::REMOVE_BY_EMAIL, &[&username]).await?;
        Ok(())
    }
    async fn get_user_by_id(&self, user_id: i32) -> Result<User> {
        let user = self.query_one(sql::SELECT_BY_ID, &[&user_id]).await?;
        user.try_into()
    }

    async fn get_user_by_username(&self, username: &str) -> Result<User> {
        let user = self.query_one(sql::SELECT_BY_EMAIL, &[&username]).await?;
        user.try_into()
    }
}

impl TryFrom<tokio_postgres::Row> for User {
    type Error = Error;
    fn try_from(row: tokio_postgres::Row) -> Result<User> {
        Ok(User {
            id: row.get(0),
            username: row.get(1),
            password: row.get(2),
            is_admin: row.get(3),
        })
    }
}
