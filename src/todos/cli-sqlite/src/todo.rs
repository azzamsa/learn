use sqlx::SqlitePool;

pub struct Todo {
    pool: SqlitePool,
}

impl Todo {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
    pub async fn add(&self, description: &str) -> Result<i64, crate::Error> {
        let id = sqlx::query!(
            r#"
INSERT INTO todos ( description )
VALUES ( ?1 )
        "#,
            description
        )
        .execute(&self.pool)
        .await?
        .last_insert_rowid();

        Ok(id)
    }
    pub async fn mark(&self, id: i64) -> Result<(), crate::Error> {
        sqlx::query!(
            r#"
        UPDATE todos
        SET done = TRUE
        WHERE id = $1
        "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
    pub async fn unmark(&self, id: i64) -> Result<(), crate::Error> {
        sqlx::query!(
            r#"
        UPDATE todos
        SET done = FALSE
        WHERE id = $1
        "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
    pub async fn remove(&self, id: i64) -> Result<(), crate::Error> {
        sqlx::query!(
            r#"
DELETE FROM todos
WHERE id = $1
        "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
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
    pub async fn description(&self, id: i64) -> Result<String, crate::Error> {
        let description = sqlx::query!(
            r#"
        SELECT description
        FROM todos
        WHERE id = $1
        "#,
            id
        )
        .fetch_one(&self.pool)
        .await?
        .description;

        Ok(description)
    }
    pub async fn status(&self, id: i64) -> Result<bool, crate::Error> {
        let done = sqlx::query!(
            r#"
        SELECT done
        FROM todos
        WHERE id = $1
        "#,
            id
        )
        .fetch_one(&self.pool)
        .await?
        .done;

        Ok(done)
    }
}
