use serde::{Deserialize, Serialize};
use sqlx::Error;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct EarnedValue {
    pub planed_value: i64,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
struct PlanedValue {
    planed_value: i64,
}

impl EarnedValue {
    pub async fn get(conn: &mut sqlx::SqliteConnection) -> Result<EarnedValue, Error> {
        let planed_value = sqlx::query_file_as!(PlanedValue, "sqls/evm/get_planed_value.sql")
            .fetch_one(conn)
            .await?;
        let mut ret = EarnedValue {
             planed_value : 0
        };
        ret.planed_value = planed_value.planed_value;
        Ok(ret)
    }
}
