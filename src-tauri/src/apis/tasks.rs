use sqlx::Acquire;
use tauri::State;
use crate::db_connection::db_connection::DbConnection;

#[tauri::command]
pub async fn task_add(connection: State<'_, DbConnection>, task_name: &str, assignee: &str) -> Result<String, String> {

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

    // TODO 追加実行
    Ok("成功だよTODO".to_string())
}

