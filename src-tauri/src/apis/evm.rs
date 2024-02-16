use sqlx::Acquire;
use tauri::State;
use crate::db_connection::db_connection::DbConnection;
use crate::models::evm::EarnedValueManagementInfo;
use crate::models::tasks::Task;

#[tauri::command]
pub async fn get_current_evm_info(connection: State<'_, DbConnection>) -> Result<EarnedValueManagementInfo, String> {
    let transaction_result =  connection.pool.begin().await;
    if transaction_result.is_err() {
        return Err(transaction_result.err().unwrap().to_string())
    }

    let mut transaction = transaction_result.unwrap();
    let conn_result = transaction.acquire().await;
    if conn_result.is_err() {
        return Err(conn_result.err().unwrap().to_string())
    }

    let conn = conn_result.ok().unwrap();

    let evm_info = EarnedValueManagementInfo::get(conn).await;
    println!("evm_info: {:?}", evm_info);

    let start_date = Task::get_min_start_date(conn).await;
    println!("start_date: {:?}", start_date);

    let get_end_date = Task::get_max_end_date(conn).await;
    println!("get_end_date: {:?}", get_end_date);

    match evm_info {
        Ok(info) => {Ok(info)}
        Err(err) => {Err(err.to_string())}
    }
}
