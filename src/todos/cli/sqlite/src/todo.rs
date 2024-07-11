use sqlx::SqlitePool;

#[derive(Debug, sqlx::FromRow)]
pub struct Todo {
    /// ID of the todo
    pub id: i64,
    /// Description of the todo
    pub description: String,
    /// Whether or not the todo has been completed.
    pub done: bool,
}

impl Todo {
    pub fn done_icon(&self) -> String {
        if self.done {
            "X".to_string()
        } else {
            " ".to_string()
        }
    }
}

pub struct Repo {
    pool: SqlitePool,
}

impl Repo {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
    pub async fn add(&self, description: &str) -> Result<Todo, crate::Error> {
        const QUERY: &str = "insert into todos (description) values ($1) returning *";

        match sqlx::query_as::<_, Todo>(QUERY)
            .bind(description)
            .fetch_one(&self.pool)
            .await
        {
            Err(err) => {
                tracing::error!("{}", &err);
                Err(err.into())
            }
            Ok(todo) => Ok(todo),
        }
    }
    pub async fn list(&self) -> Result<Vec<Todo>, crate::Error> {
        const QUERY: &str = "select * from todos order by id";

        match sqlx::query_as::<_, Todo>(QUERY).fetch_all(&self.pool).await {
            Err(err) => {
                tracing::error!("{}", &err);
                Err(err.into())
            }
            Ok(todo) => Ok(todo),
        }
    }
    pub async fn toggle(&self, id: i64) -> Result<Todo, crate::Error> {
        let mut todo = self.get(id).await?;
        todo.done = !todo.done;
        self.update(todo).await
    }
    pub async fn remove(&self, id: i64) -> Result<Todo, crate::Error> {
        const QUERY: &str = "delete * from todos where id = $1 returning *";

        match sqlx::query_as::<_, Todo>(QUERY)
            .bind(id)
            .fetch_one(&self.pool)
            .await
        {
            Err(err) => {
                tracing::error!("{}", &err);
                Err(err.into())
            }
            Ok(todo) => Ok(todo),
        }
    }
    pub async fn get(&self, id: i64) -> Result<Todo, crate::Error> {
        const QUERY: &str = "SELECT * FROM todos WHERE id = $1";

        match sqlx::query_as::<_, Todo>(QUERY)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
        {
            Err(err) => {
                tracing::error!("{}", &err);
                Err(err.into())
            }
            Ok(None) => Err(crate::error::AppError::TodoNotFound.into()),
            Ok(Some(res)) => Ok(res),
        }
    }
    async fn update(&self, todo: Todo) -> Result<Todo, crate::Error> {
        const QUERY: &str = "update todos set
              description = $2,
              done = $3,
           where id = $1 returning *";

        match sqlx::query_as::<_, Todo>(QUERY)
            .bind(todo.id)
            .bind(todo.description)
            .bind(todo.done)
            .fetch_optional(&self.pool)
            .await
        {
            Err(err) => {
                tracing::error!("{}", &err);
                Err(err.into())
            }
            Ok(None) => Err(crate::error::AppError::TodoNotFound.into()),
            Ok(Some(res)) => Ok(res),
        }
    }
}
