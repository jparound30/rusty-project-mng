use serde::{Deserialize, Serialize};
use sqlx::Error;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct EarnedValueManagementInfo {
    pub planed_value: i64,
    pub earned_value: i64,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
struct PlanedValue {
    planed_value: i64,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
struct EarnedValue {
    earned_value: i64,
}

impl EarnedValueManagementInfo {
    pub async fn get(conn: &mut sqlx::SqliteConnection) -> Result<EarnedValueManagementInfo, Error> {
        let planed_value = sqlx::query_file_as!(PlanedValue, "sqls/evm/get_planed_value.sql")
            .fetch_one(&mut *conn)
            .await?;

        let earned_value = sqlx::query_file_as!(EarnedValue, "sqls/evm/get_earned_value.sql", 80, 10)
            .fetch_one(&mut *conn)
            .await?;

        let mut ret = EarnedValueManagementInfo {
            planed_value: 0,
            earned_value: 0,
        };
        ret.planed_value = planed_value.planed_value;
        ret.earned_value = earned_value.earned_value;
        Ok(ret)
    }
}
