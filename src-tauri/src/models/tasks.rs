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
    pub planned_value: Option<i64>,
    pub task_status_id: i64,
    pub progress_rate: i64,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct TaskFull {
    pub task_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub assignee_resource_id: Option<i64>,
    pub assignee_resource_name: Option<String>,
    pub parent_task_id: Option<i64>,
    pub parent_task_title: Option<String>,
    pub start_date: Option<String>,
    pub due_date: Option<String>,
    pub estimated_time: Option<i64>,
    pub actual_time: Option<i64>,
    pub planned_value: Option<i64>,
    pub task_status_id: i64,
    pub task_status_name: String,
    pub progress_rate: i64,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct TaskSimple {
    pub task_id: i64,
    pub title: String,
}

impl Task {
    pub async fn add(mut self: Self, conn: &mut sqlx::SqliteConnection) -> Result<(), Error> {
        let last_insert_rowid = sqlx::query_file!(
            "sqls/tasks/add.sql",
            self.title,
            self.description,
            self.assignee_resource_id,
            self.parent_task_id,
            self.start_date,
            self.due_date,
            self.estimated_time,
            self.actual_time,
            self.planned_value,
            self.task_status_id,
            self.progress_rate,
        )
            .execute(conn)
            .await?
            .last_insert_rowid();

        self.task_id = last_insert_rowid;
        Ok(())
    }

    pub async fn update(mut self: Self, conn: &mut sqlx::SqliteConnection) -> Result<(), Error> {
        let last_insert_rowid = sqlx::query_file!(
            "sqls/tasks/update.sql",
            self.task_id,
            self.title,
            self.description,
            self.assignee_resource_id,
            self.parent_task_id,
            self.start_date,
            self.due_date,
            self.estimated_time,
            self.actual_time,
            self.planned_value,
            self.task_status_id,
            self.progress_rate,
        )
            .execute(conn)
            .await?
            .last_insert_rowid();

        self.task_id = last_insert_rowid;
        Ok(())
    }

    pub async fn delete(conn: &mut sqlx::SqliteConnection, task_id: i64) -> Result<(), Error> {
        sqlx::query_file!("sqls/tasks/delete.sql", task_id)
            .execute(conn)
            .await?;
        Ok(())
    }

    pub async fn get_full(conn: &mut sqlx::SqliteConnection, task_id: i64) -> Result<Option<TaskFull>, Error> {
        let option = sqlx::query_file_as!(TaskFull, "sqls/tasks/get_full.sql", task_id)
            .fetch_optional(conn)
            .await?;
        Ok(option)
    }

    pub async fn all(conn: &mut sqlx::SqliteConnection) -> Result<Vec<TaskFull>, Error> {
        let option = sqlx::query_file_as!(TaskFull, "sqls/tasks/all_full.sql")
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

    pub async fn get_min_start_date(conn: &mut sqlx::SqliteConnection) -> Result<String, Error> {
        // type SqlDate = NaiveDate;
        let record = sqlx::query_file!("sqls/tasks/get_min_start_date.sql")
            .fetch_one(conn)
            .await?;
        Ok(record.start_date.expect("get_min_start_date failed"))
    }

    pub async fn get_max_end_date(conn: &mut sqlx::SqliteConnection) -> Result<String, Error> {
        let record = sqlx::query_file!("sqls/tasks/get_max_end_date.sql")
            .fetch_one(conn)
            .await?;
        Ok(record.end_date.expect("get_max_end_date failed"))
    }
}
