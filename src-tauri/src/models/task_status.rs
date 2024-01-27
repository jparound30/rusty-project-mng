use serde::{Deserialize, Serialize};
use sqlx::Error;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct TaskStatus {
    pub task_status_id: u32,
    pub title: String,
    pub view_order: u32,
}


impl TaskStatus {
    pub async fn all(conn: &mut sqlx::SqliteConnection) -> Result<Vec<TaskStatus>, Error> {
        let list = sqlx::query_as::<_, TaskStatus>(
            r#"
            SELECT task_status_id, title, view_order
            FROM task_status
            "#)
            .fetch_all(conn)
            .await?;
        Ok(list)
    }
}