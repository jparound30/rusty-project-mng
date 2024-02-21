use sqlx::Acquire;
use tauri::State;
use crate::db_connection::db_connection::DbConnection;
use crate::models::evm::{EarnedValueManagementInfo, PlanedValueChanges};
use crate::models::evm_histories::EvmHistory;
use crate::models::tasks::Task;

#[tauri::command]
pub async fn get_current_evm_info(connection: State<'_, DbConnection>) -> Result<EarnedValueManagementInfo, String> {
    let transaction_result = connection.pool.begin().await;
    if transaction_result.is_err() {
        return Err(transaction_result.err().unwrap().to_string());
    }

    let mut transaction = transaction_result.unwrap();
    let conn_result = transaction.acquire().await;
    if conn_result.is_err() {
        return Err(conn_result.err().unwrap().to_string());
    }

    let conn = conn_result.ok().unwrap();

    let evm_info = EarnedValueManagementInfo::get(conn).await;
    println!("evm_info: {:?}", evm_info);

    match evm_info {
        Ok(info) => { Ok(info) }
        Err(err) => { Err(err.to_string()) }
    }
}

#[tauri::command]
pub async fn get_planned_value_changes(connection: State<'_, DbConnection>) -> Result<Vec<PlanedValueChanges>, String> {
    let transaction_result = connection.pool.begin().await;
    if transaction_result.is_err() {
        return Err(transaction_result.err().unwrap().to_string());
    }

    let mut transaction = transaction_result.unwrap();
    let conn_result = transaction.acquire().await;
    if conn_result.is_err() {
        return Err(conn_result.err().unwrap().to_string());
    }

    let conn = conn_result.ok().unwrap();

    let start_date = Task::get_min_start_date(conn).await;
    println!("start_date: {:?}", start_date);

    let end_date = Task::get_max_end_date(conn).await;
    println!("end_date: {:?}", end_date);


    let planned_value_changes =
        EarnedValueManagementInfo::get_planned_value_changes(
            conn,
            &start_date.unwrap(),
            &end_date.unwrap()).await;
    println!("planned_value_changes: {:?}", planned_value_changes);

    match planned_value_changes {
        Ok(info) => { Ok(info) }
        Err(err) => { Err(err.to_string()) }
    }
}

#[tauri::command]
pub async fn get_evm_histories(connection: State<'_, DbConnection>) -> Result<Vec<EvmHistory>, String> {
    let transaction_result = connection.pool.begin().await;
    if transaction_result.is_err() {
        return Err(transaction_result.err().unwrap().to_string());
    }

    let mut transaction = transaction_result.unwrap();
    let conn_result = transaction.acquire().await;
    if conn_result.is_err() {
        return Err(conn_result.err().unwrap().to_string());
    }

    let conn = conn_result.ok().unwrap();

    let evm_histories =
        EvmHistory::get_all(conn).await;
    println!("evm_histories: {:?}", evm_histories);

    match evm_histories {
        Ok(info) => { Ok(info) }
        Err(err) => { Err(err.to_string()) }
    }
}
