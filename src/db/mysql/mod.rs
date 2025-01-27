use crate::prelude::{Result, *};
mod sql;
use sql::*;

use sqlx::mysql::MySqlPool;

use sqlx::*;

#[rocket::async_trait]
impl DBConnection for MySqlPool {
    async fn init(&self) -> Result<()> {
        query(CREATE_TABLE).execute(self).await?;
        Ok(())
    }
    async fn create_user(&self, username: &str, hash: &str, is_admin: bool) -> Result<()> {
        query(INSERT_USER)
            .bind(username)
            .bind(hash)
            .bind(is_admin)
            .execute(self)
            .await?;
        Ok(())
    }
    async fn update_user(&self, user: &User) -> Result<()> {
        query(UPDATE_USER)
            .bind(&user.username)
            .bind(&user.password)
            .bind(user.is_admin)
            .bind(user.id)
            .execute(self)
            .await?;

        Ok(())
    }
    async fn delete_user_by_id(&self, user_id: i32) -> Result<()> {
        query(REMOVE_BY_ID).bind(user_id).execute(self).await?;
        Ok(())
    }
    async fn delete_user_by_username(&self, username: &str) -> Result<()> {
        query(REMOVE_BY_EMAIL).bind(username).execute(self).await?;
        Ok(())
    }
    async fn get_user_by_id(&self, user_id: i32) -> Result<User> {
        let user = query_as(SELECT_BY_ID).bind(user_id).fetch_one(self).await?;

        Ok(user)
    }
    async fn get_user_by_username(&self, username: &str) -> Result<User> {
        let user = query_as(SELECT_BY_EMAIL)
            .bind(username)
            .fetch_one(self)
            .await?;
        Ok(user)
    }
}
