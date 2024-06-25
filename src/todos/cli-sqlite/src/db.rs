use sqlx::SqlitePool;

use crate::config;

pub async fn connect(db: &config::Database) -> Result<SqlitePool, crate::Error> {
    Ok(SqlitePool::connect(&db.url).await?)
}

pub async fn migrate(pool: &SqlitePool) -> Result<(), crate::Error> {
    let mut conn = pool.acquire().await?;
    Ok(sqlx::migrate!().run(&mut conn).await?)
}
