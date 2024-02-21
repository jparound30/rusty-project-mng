use serde::{Deserialize, Serialize};
use sqlx::Error;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct EvmHistory {
    pub date: String,
    pub actual_cost: i64,
    pub earned_value: i64,
}

impl EvmHistory {
    pub async fn get_all(conn: &mut sqlx::SqliteConnection) -> Result<Vec<EvmHistory>, Error> {
        let evm_histories = sqlx::query_file_as!(EvmHistory, "sqls/evm_histories/all.sql")
            .fetch_all(&mut *conn)
            .await?;


        Ok(evm_histories)
    }
}
