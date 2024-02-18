use serde::{Deserialize, Serialize};
use sqlx::Error;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct TaskStatus {
    pub task_status_id: i64,
    pub title: String,
    pub view_order: i64,
}


impl TaskStatus {
    pub async fn all(conn: &mut sqlx::SqliteConnection) -> Result<Vec<TaskStatus>, Error> {
        let list = sqlx::query_file_as!(TaskStatus, "sqls/task_status/get_all.sql")
            .fetch_all(conn)
            .await?;
        Ok(list)
    }
}