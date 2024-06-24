use sqlx::{pool::PoolConnection, Sqlite, SqlitePool};

pub struct Todo {
    pool: SqlitePool,
}

impl Todo {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
    pub async fn add(&self, description: &str) -> Result<i64, crate::Error> {
        let mut conn = self.conn().await?;
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
    pub async fn mark(&self, id: i64) -> Result<bool, crate::Error> {
        let rows_affected = sqlx::query!(
            r#"
UPDATE todos
SET done = TRUE
WHERE id = ?1
        "#,
            id
        )
        .execute(&self.pool)
        .await?
        .rows_affected();

        Ok(rows_affected > 0)
    }
    pub async fn unmark(&self, id: i64) -> Result<bool, crate::Error> {
        let rows_affected = sqlx::query!(
            r#"
UPDATE todos
SET done = FALSE
WHERE id = ?1
        "#,
            id
        )
        .execute(&self.pool)
        .await?
        .rows_affected();

        Ok(rows_affected > 0)
    }
    pub async fn list(&self) -> Result<(), crate::Error> {
        let recs = sqlx::query!(
            r#"
SELECT id, description, done
FROM todos
ORDER BY id
        "#
        )
        .fetch_all(&self.pool)
        .await?;

        for rec in recs {
            println!(
                "- [{}] {}: {}",
                if rec.done { "X" } else { "" },
                rec.id,
                &rec.description,
            );
        }

        Ok(())
    }
    async fn conn(&self) -> Result<PoolConnection<Sqlite>, crate::Error> {
        Ok(self.pool.acquire().await?)
    }
}
