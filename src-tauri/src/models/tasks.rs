use sqlx::Error;

#[derive(Debug, sqlx::FromRow)]
pub struct Task {
    pub task_id: u32,
    pub title: String,
    pub description: Option<String>,
    pub assignee_resource_id: Option<u32>,
    pub parent_task_id: Option<u32>,
    pub start_date: Option<String>,
    pub due_date: Option<String>,
    pub estimated_time: Option<u32>,
    pub actual_time: Option<u32>,
    pub planed_value: Option<u32>,
    pub task_status_id: u32,
    pub progress_rate: u32,
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

        self.task_id = last_insert_rowid as u32;
        Ok(())
    }

    pub async fn get(conn: &mut sqlx::SqliteConnection, task_id: u32) -> Result<Option<Task>, Error> {
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
}

#[allow(dead_code)]
#[derive(Debug, sqlx::FromRow)]
struct UserAndAuth {
    user_id: i32,
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
