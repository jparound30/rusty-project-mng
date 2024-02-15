use serde::{Deserialize, Serialize};
use sqlx::Error;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct EarnedValueManagementInfo {
    pub planned_value: i64,
    pub earned_value: i64,
    pub actual_cost: i64,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
struct PlanedValue {
    planned_value: i64,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
struct EarnedValue {
    earned_value: i64,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
struct ActualCost {
    actual_cost: i64,
}

impl EarnedValueManagementInfo {
    pub async fn get(conn: &mut sqlx::SqliteConnection) -> Result<EarnedValueManagementInfo, Error> {
        let planed_value = sqlx::query_file_as!(PlanedValue, "sqls/evm/get_planned_value.sql")
            .fetch_one(&mut *conn)
            .await?;

        let earned_value = sqlx::query_file_as!(EarnedValue, "sqls/evm/get_earned_value.sql", 80, 10)
            .fetch_one(&mut *conn)
            .await?;

        let actual_cost = sqlx::query_file_as!(ActualCost, "sqls/evm/get_actual_cost.sql")
            .fetch_one(&mut *conn)
            .await?;

        let ret = EarnedValueManagementInfo {
            planned_value: planed_value.planned_value,
            earned_value: earned_value.earned_value,
            actual_cost: actual_cost.actual_cost,
        };
        Ok(ret)
    }
}
