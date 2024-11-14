use sqlx::{FromRow, Pool, Postgres};
use anyhow::Result;
use rocket_db_pools::Connection;
use crate::Db;

#[derive(Debug)]
pub struct User {
    pub name: String,
    pub image_url: String,
    pub user_id: String,
}

impl User {
    pub async fn find_by_id(user_id: String, mut db: Connection<Db>) -> Result<User> {
        let user: User = sqlx::query_as!(User, "SELECT * FROM users WHERE user_id = $1", user_id)
            .fetch_one(&mut **db)
            .await?;
        Ok(user)
    }
    
}
