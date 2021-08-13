mod sql;
use std::convert::{TryFrom, TryInto};

use crate::prelude::{Result, *};
use rocket::async_trait;
use sql::*;
use tokio::sync::Mutex;

use rusqlite::*;

#[cfg(feature = "rusqlite")]
use rusqlite::Row;
#[cfg(feature = "rusqlite")]
impl<'a> TryFrom<&rusqlite::Row<'a>> for crate::User {
    type Error = rusqlite::Error;
    fn try_from(row: &Row) -> Result<User, rusqlite::Error> {
        Ok(User {
            id: row.get(0)?,
            email: row.get(1)?,
            password: row.get(2)?,
            is_admin: row.get(3)?,
        })
    }
}

#[cfg(feature = "rusqlite")]
#[async_trait]
impl DBConnection for Mutex<rusqlite::Connection> {
    async fn init(&self) -> Result<()> {
        let conn = self.lock().await;
        conn.execute(sql::CREATE_TABLE, [])?;
        Ok(())
    }

    async fn create_user(&self, email: &str, hash: &str, is_admin: bool) -> Result<()> {
        let conn = self.lock().await;
        conn.execute(sql::INSERT_USER, params![email, hash, is_admin])?;
        Ok(())
    }

    async fn update_user(&self, user: &User) -> Result<()> {
        let conn = self.lock().await;
        conn.execute(
            sql::UPDATE_USER,
            params![user.id, user.email, user.password, user.is_admin],
        )?;
        Ok(())
    }

    async fn delete_user_by_id(&self, user_id: i32) -> Result<()> {
        let conn = self.lock().await;
        conn.execute(sql::REMOVE_BY_ID, params![user_id])?;
        Ok(())
    }

    async fn delete_user_by_email(&self, email: &str) -> Result<()> {
        let conn = self.lock().await;
        conn.execute(sql::REMOVE_BY_EMAIL, params![email])?;
        Ok(())
    }

    async fn get_user_by_id(&self, user_id: i32) -> Result<User> {
        let conn = self.lock().await;
        let user = conn.query_row(
            sql::SELECT_BY_ID, //
            params![user_id],
            |row| row.try_into(),
        )?;
        Ok(user)
    }

    async fn get_user_by_email(&self, email: &str) -> Result<User> {
        let conn = self.lock().await;
        let user = conn.query_row(
            sql::SELECT_BY_EMAIL, //
            params![email],
            |row| row.try_into(),
        )?;
        Ok(user)
    }
}

#[cfg(feature = "sqlite-db")]
use sqlx::{sqlite::SqliteConnection, *};
#[cfg(feature = "sqlite-db")]
#[rocket::async_trait]
impl DBConnection for Mutex<SqliteConnection> {
    async fn init(&self) -> Result<()> {
        let mut db = self.lock().await;
        query(CREATE_TABLE).execute(&mut *db).await?;
        println!("table created");
        Ok(())
    }
    async fn create_user(&self, email: &str, hash: &str, is_admin: bool) -> Result<()> {
        let mut db = self.lock().await;
        query(INSERT_USER)
            .bind(email)
            .bind(hash)
            .bind(is_admin)
            .execute(&mut *db)
            .await?;
        Ok(())
    }
    async fn update_user(&self, user: &User) -> Result<()> {
        let mut db = self.lock().await;
        query(UPDATE_USER)
            .bind(user.id)
            .bind(&user.email)
            .bind(&user.password)
            .bind(user.is_admin)
            .execute(&mut *db)
            .await?;
        Ok(())
    }
    async fn delete_user_by_id(&self, user_id: i32) -> Result<()> {
        query(REMOVE_BY_ID)
            .bind(user_id)
            .execute(&mut *self.lock().await)
            .await?;
        Ok(())
    }
    async fn delete_user_by_email(&self, email: &str) -> Result<()> {
        query(REMOVE_BY_EMAIL)
            .bind(email)
            .execute(&mut *self.lock().await)
            .await?;
        Ok(())
    }
    async fn get_user_by_id(&self, user_id: i32) -> Result<User> {
        let mut db = self.lock().await;

        let user = query_as(SELECT_BY_ID)
            .bind(user_id)
            .fetch_one(&mut *db)
            .await?;

        Ok(user)
    }
    async fn get_user_by_email(&self, email: &str) -> Result<User> {
        let mut db = self.lock().await;
        let user = query_as(SELECT_BY_EMAIL)
            .bind(email)
            .fetch_one(&mut *db)
            .await?;
        Ok(user)
    }
}

#[rocket::async_trait]
impl DBConnection for SqlitePool {
    async fn init(&self) -> Result<()> {
        query(CREATE_TABLE) //
            .execute(self)
            .await?;
        Ok(())
    }
    async fn create_user(&self, email: &str, hash: &str, is_admin: bool) -> Result<()> {
        query(INSERT_USER)
            .bind(email)
            .bind(hash)
            .bind(is_admin)
            .execute(self)
            .await?;
        Ok(())
    }
    async fn update_user(&self, user: &User) -> Result<()> {
        query(UPDATE_USER)
            .bind(user.id)
            .bind(&user.email)
            .bind(&user.password)
            .bind(user.is_admin)
            .execute(self)
            .await?;
        Ok(())
    }
    async fn delete_user_by_id(&self, user_id: i32) -> Result<()> {
        query(REMOVE_BY_ID) //
            .bind(user_id)
            .execute(self)
            .await?;
        Ok(())
    }
    async fn delete_user_by_email(&self, email: &str) -> Result<()> {
        query(REMOVE_BY_EMAIL) //
            .bind(email)
            .execute(self)
            .await?;
        Ok(())
    }
    async fn get_user_by_id(&self, user_id: i32) -> Result<User> {
        let user = query_as(SELECT_BY_ID) //
            .bind(user_id)
            .fetch_one(self)
            .await?;
        Ok(user)
    }
    async fn get_user_by_email(&self, email: &str) -> Result<User> {
        let user = query_as(SELECT_BY_EMAIL).bind(email).fetch_one(self).await;
        println!("user: {:?}", user);
        Ok(user?)
    }
}
