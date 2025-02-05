use sqlx::{sqlite::SqliteConnectOptions, Error, SqlitePool};
use std::{env, path::Path};

async fn connect(filename: impl AsRef<Path>) -> Result<SqlitePool, Error> {
    let options = SqliteConnectOptions::new()
        .filename(filename)
        .create_if_missing(true); // Create the database file if it doesn't exist

    let pool = SqlitePool::connect_with(options).await?;
    println!("Connected to the database successfully.");
    Ok(pool)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let is_setup = args.contains(&"--setup".to_string());

    let pool = connect("sql.db").await?;

    // Create schema
    match sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            country TEXT NOT NULL,
            organization TEXT NOT NULL
        )"#,
    )
    .execute(&pool)
    .await
    {
        Ok(_) => println!("Users table created or already exists."),
        Err(err) => eprintln!("Error creating users table: {}", err),
    }

    if is_setup {
        // Insert base data
        let data = [
            ("USA", "01234567890ABCDEF"),
            ("Canada", "01234567890ABCDEF"),
            ("USA", "ABCDEF1234567890"),
        ];

        for (country, org) in data {
            match sqlx::query("INSERT INTO users (country, organization) VALUES (?, ?)")
                .bind(country)
                .bind(org)
                .execute(&pool)
                .await
            {
                Ok(_) => println!("Inserted data: {} - {}", country, org),
                Err(err) => eprintln!("Error inserting data: {}", err),
            }
        }
        return Ok(());
    }
    // Regular query execution
    let organization = "01234567890ABCDEF";
    let countries = match sqlx::query!(
        r#"SELECT country, COUNT(*) as count
           FROM users
           WHERE organization = ?
           GROUP BY country"#,
        organization
    )
    .fetch_all(&pool)
    .await
    {
        Ok(records) => records,
        Err(err) => {
            eprintln!("Error fetching data: {}", err);
            return Err(err);
        }
    };

    println!("Country counts for '{}':", organization);
    for record in countries {
        println!("- {}: {}", record.country, record.count);
    }

    Ok(())
}
