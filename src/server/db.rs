use sqlx::{sqlite::SqliteConnectOptions, Error, SqlitePool, Row};
use std::path::Path;
use super::crypt::PasswordHasher;

pub struct User {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
}

#[derive(Debug)]
pub enum DatabaseError {
    SqlxError(Error),
    UserExists,
    InvalidCredentials,
}

impl From<Error> for DatabaseError {
    fn from(err: Error) -> Self {
        DatabaseError::SqlxError(err)
    }
}

pub struct Database {
    pool: SqlitePool,
    password_hasher: PasswordHasher,
}

impl Database {
    pub async fn connect(filename: impl AsRef<Path>) -> Result<Self, Error> {
        let options = SqliteConnectOptions::new()
            .filename(filename)
            .create_if_missing(true);

        let pool = SqlitePool::connect_with(options).await?;
        println!("Connected to the database successfully.");

        let db = Database { 
            pool,
            password_hasher: PasswordHasher::new(),
        };
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
                password_hash TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn init_test_user(&self) -> Result<(), Error> {
        let hashed_password = self.password_hasher.hash_password("password123");
        sqlx::query(
            r#"
            INSERT OR IGNORE INTO users (username, password_hash)
            VALUES ('admin', ?)
            "#,
        )
        .bind(hashed_password)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn verify_user(&self, username: &str, password: &str) -> Result<bool, DatabaseError> {
        let result = sqlx::query(
            "SELECT password_hash FROM users WHERE username = ?"
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await?;

        match result {
            Some(row) => {
                let stored_hash: String = row.get(0);
                Ok(self.password_hasher.verify_password(password, &stored_hash))
            }
            None => Ok(false),
        }
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

        // Hash password and insert new user
        let hashed_password = self.password_hasher.hash_password(password);
        sqlx::query(
            "INSERT INTO users (username, password_hash) VALUES (?, ?)"
        )
        .bind(username)
        .bind(hashed_password)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_user(&self, username: &str) -> Result<Option<User>, DatabaseError> {
        let row = sqlx::query(
            "SELECT id, username, password_hash FROM users WHERE username = ?"
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => Ok(Some(User {
                id: row.get(0),
                username: row.get(1),
                password_hash: row.get(2),
            })),
            None => Ok(None),
        }
    }
}
