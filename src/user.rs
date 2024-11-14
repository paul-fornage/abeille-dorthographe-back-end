use sqlx::{Error, FromRow, Pool, Postgres};
use anyhow::Result;
use rocket_db_pools::Connection;
use crate::Db;

#[derive(Debug, FromRow, PartialEq)]
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
    
    pub async fn add_to_db(self, mut db: Connection<Db>) -> Result<()> {
        sqlx::query_as!(User, 
            "INSERT INTO users (user_id, name, image_url)
             VALUES ($1, $2, $3);", 
            self.user_id, self.name, self.image_url)
            .fetch_one(&mut **db)
            .await?;
        Ok(())
    }

    pub async fn find_or_create(user: User, mut db: Connection<Db>) -> Result<User> {
        match sqlx::query_as!(User, "SELECT * FROM users WHERE user_id = $1", user.user_id)
            .fetch_one(&mut **db)
            .await {
            Ok(user_from_server) => {
                if user == user_from_server {
                    Ok(user)
                } else {
                    println!("Database user info does not agree with google, likely user changed google name or image since last sign in.");
                    Ok(user) // TODO: update data in db to reflect new info
                }
            }
            Err(_) => {
                sqlx::query!(
                    "INSERT INTO users (user_id, name, image_url)
                     VALUES ($1, $2, $3);", 
                    user.user_id, user.name, user.image_url)
                    .execute(&mut **db)
                    .await?;
                Ok(user)
            }
        }
    }
    
}
