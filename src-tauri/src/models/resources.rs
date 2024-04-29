use serde::{Deserialize, Serialize};
use sqlx::Error;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Resource {
    pub resource_id: i64,
    pub name: String,
    pub cost_per_month: i64,
}


impl Resource {
    pub async fn all(conn: &mut sqlx::SqliteConnection) -> Result<Vec<Resource>, Error> {
        let list = sqlx::query_file_as!(Resource, "sqls/resources/get_all.sql")
            .fetch_all(conn)
            .await?;
        Ok(list)
    }

    pub async fn add(mut self: Self, conn: &mut sqlx::SqliteConnection) -> Result<(), Error> {
        let last_insert_row_id = sqlx::query_file!(
            "sqls/resources/add.sql",
            self.name,
            self.cost_per_month
        )
            .execute(conn)
            .await?
            .last_insert_rowid();

        self.resource_id = last_insert_row_id;
        Ok(())
    }
    pub async fn update(self: Self, conn: &mut sqlx::SqliteConnection) -> Result<(), Error> {
        sqlx::query_file!(
            "sqls/resources/update.sql",
            self.name,
            self.cost_per_month,
            self.resource_id,
        )
            .execute(conn)
            .await?;
        Ok(())
    }
}
