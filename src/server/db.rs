use sqlx::{sqlite::SqliteConnectOptions, Error, SqlitePool};
use std::path::Path;

pub struct User {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
}

#[derive(Debug)]
pub enum DatabaseError {
    SqlxError(Error),
    UserExists,
}

impl From<Error> for DatabaseError {
    fn from(err: Error) -> Self {
        DatabaseError::SqlxError(err)
    }
}

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn connect(filename: impl AsRef<Path>) -> Result<Self, Error> {
        let options = SqliteConnectOptions::new()
            .filename(filename)
            .create_if_missing(true);

        let pool = SqlitePool::connect_with(options).await?;
        println!("Connected to the database successfully.");

        let db = Database { pool };
        db.init_tables().await?;
        db.init_test_user().await?;
        
        Ok(db)
    }

    async fn init_tables(&self) -> Result<(), Error> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY,
                username TEXT NOT NULL UNIQUE,
                password_hash TEXT NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn init_test_user(&self) -> Result<(), Error> {
        sqlx::query(
            r#"
            INSERT OR IGNORE INTO users (username, password_hash)
            VALUES ('admin', 'password123')
            "#,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn verify_user(&self, username: &str, password: &str) -> Result<bool, Error> {
        let result = sqlx::query(
            "SELECT id FROM users WHERE username = ? AND password_hash = ?"
        )
        .bind(username)
        .bind(password)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.is_some())
    }

    pub async fn register_user(&self, username: &str, password: &str) -> Result<(), DatabaseError> {
        // Check if user already exists
        let exists = sqlx::query(
            "SELECT id FROM users WHERE username = ?"
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await?;

        if exists.is_some() {
            return Err(DatabaseError::UserExists);
        }

        // Insert new user
        sqlx::query(
            "INSERT INTO users (username, password_hash) VALUES (?, ?)"
        )
        .bind(username)
        .bind(password)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
