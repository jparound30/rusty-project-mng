use serde::{Deserialize, Serialize};
use sqlx::Error;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Resources {
    pub resource_id: u32,
    pub name: String,
    pub cost_per_month: u32,
}


impl Resources {
    pub async fn all(conn: &mut sqlx::SqliteConnection) -> Result<Vec<Resources>, Error> {
        let list = sqlx::query_as::<_, Resources>(
            r#"
            SELECT resource_id, name, cost_per_month
            FROM resources
            "#)
            .fetch_all(conn)
            .await?;
        Ok(list)
    }
}