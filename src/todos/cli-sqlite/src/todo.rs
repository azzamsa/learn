use sqlx::SqlitePool;

pub async fn add(pool: &SqlitePool, description: &str) -> Result<i64, crate::Error> {
    let mut conn = pool.acquire().await?;

    // Insert the task, then obtain the ID of this row
    let id = sqlx::query!(
        r#"
INSERT INTO todos ( description )
VALUES ( ?1 )
        "#,
        description
    )
    .execute(&mut *conn)
    .await?
    .last_insert_rowid();

    Ok(id)
}
