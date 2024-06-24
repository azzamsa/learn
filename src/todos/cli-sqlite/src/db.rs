use std::env;

use sqlx::{Connection, SqliteConnection};

pub async fn connect() -> Result<SqliteConnection, crate::Error> {
    Ok(SqliteConnection::connect(&env::var("DATABASE_URL")?).await?)
}

pub async fn migrate(mut conn: SqliteConnection) -> Result<(), crate::Error> {
    Ok(sqlx::migrate!("db/migrations").run(&mut conn).await?)
}
