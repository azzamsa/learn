use std::env;

use sqlx::SqlitePool;

pub async fn connect() -> Result<SqlitePool, crate::Error> {
    Ok(SqlitePool::connect(&env::var("DATABASE_URL")?).await?)
}

pub async fn migrate(pool: &SqlitePool) -> Result<(), crate::Error> {
    let mut conn = pool.acquire().await?;
    Ok(sqlx::migrate!().run(&mut conn).await?)
}
