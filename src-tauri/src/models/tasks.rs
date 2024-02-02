use serde::{Deserialize, Serialize};
use sqlx::Error;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Task {
    pub task_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub assignee_resource_id: Option<i64>,
    pub parent_task_id: Option<i64>,
    pub start_date: Option<String>,
    pub due_date: Option<String>,
    pub estimated_time: Option<i64>,
    pub actual_time: Option<i64>,
    pub planed_value: Option<i64>,
    pub task_status_id: i64,
    pub progress_rate: i64,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct TaskSimple {
    pub task_id: i64,
    pub title: String,
}

impl Task {
    pub async fn add(mut self: Self, conn: &mut sqlx::SqliteConnection) -> Result<(), Error> {
        let last_insert_rowid = sqlx::query(
            "INSERT INTO tasks \
                    (title, description, assignee_resource_id, parent_task_id, start_date, due_date, estimated_time, actual_time, planed_value, task_status_id, progress_rate)\
                 VALUES\
                    (?1,?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)")
            .bind(self.title)
            .bind(self.description)
            .bind(self.assignee_resource_id)
            .bind(self.parent_task_id)
            .bind(self.start_date)
            .bind(self.due_date)
            .bind(self.estimated_time)
            .bind(self.actual_time)
            .bind(self.planed_value)
            .bind(self.task_status_id)
            .bind(self.progress_rate)
            .execute(conn)
            .await?
            .last_insert_rowid();

        self.task_id = last_insert_rowid as i64;
        Ok(())
    }

    pub async fn get(conn: &mut sqlx::SqliteConnection, task_id: i64) -> Result<Option<Task>, Error> {
        let option = sqlx::query_as::<_, Task>("\
                        SELECT task_id, title, description, assignee_resource_id, parent_task_id, start_date, due_date, estimated_time, actual_time, planed_value, task_status_id, progress_rate\
                        FROM tasks \
                        WHERE task_id = ?1
                        ")
            .bind(task_id)
            .fetch_optional(conn)
            .await?;
        Ok(option)
    }

    pub async fn all(conn: &mut sqlx::SqliteConnection) -> Result<Vec<Task>, Error> {
        let option = sqlx::query_file_as!(Task, "sqls/tasks/all.sql")
            .fetch_all(conn)
            .await?;
        Ok(option)
    }

    pub async fn all_with_id_and_title(conn: &mut sqlx::SqliteConnection) -> Result<Vec<TaskSimple>, Error> {
        let option = sqlx::query_file_as!(TaskSimple, "sqls/tasks/all_with_id_and_title.sql")
            .fetch_all(conn)
            .await?;
        Ok(option)
    }

}

#[allow(dead_code)]
#[derive(Debug, sqlx::FromRow)]
struct UserAndAuth {
    user_id: i64,
    username: String,
    password_hash: String,
    salt: String,
}

#[allow(dead_code)]
pub async fn show_all(conn: &mut sqlx::SqliteConnection) -> Result<(), Error> {
    let users = sqlx::query_as::<_, UserAndAuth>("SELECT user_id, username, password_hash, salt FROM users")
        .fetch_all(conn)
        .await?;

    for user in users {
        println!("Found user {:?}", user);
    }
    Ok(())
}
