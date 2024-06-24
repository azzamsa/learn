use sqlx::SqlitePool;

pub struct Todo {
    pool: SqlitePool,
}

impl Todo {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
    pub async fn add(&self, description: &str) -> Result<(), crate::Error> {
        sqlx::query!(
            r#"
INSERT INTO todos ( description )
VALUES ( ?1 )
        "#,
            description
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
    pub async fn mark(&self, id: i64) -> Result<String, crate::Error> {
        let description = sqlx::query!(
            r#"
UPDATE todos
SET done = TRUE
WHERE id = ?1
RETURNING description
        "#,
            id
        )
        .fetch_one(&self.pool)
        .await?
        .description;

        Ok(description)
    }
    pub async fn unmark(&self, id: i64) -> Result<String, crate::Error> {
        let description = sqlx::query!(
            r#"
UPDATE todos
SET done = FALSE
WHERE id = ?1
RETURNING description
        "#,
            id
        )
        .fetch_one(&self.pool)
        .await?
        .description;

        Ok(description)
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
}
