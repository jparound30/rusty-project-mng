use serde::{Deserialize, Serialize};
use sqlx::Error;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Resources {
    pub resource_id: i64,
    pub name: String,
    pub cost_per_month: i64,
}


impl Resources {
    pub async fn all(conn: &mut sqlx::SqliteConnection) -> Result<Vec<Resources>, Error> {
        let list = sqlx::query_file_as!(Resources, "sqls/resources/get_all.sql")
            .fetch_all(conn)
            .await?;
        Ok(list)
    }
}