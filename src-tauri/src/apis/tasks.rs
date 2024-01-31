use sqlx::Acquire;
use tauri::State;
use crate::db_connection::db_connection::DbConnection;

#[tauri::command]
pub async fn task_add(connection: State<'_, DbConnection>, title: &str, assignee_resource_id: Option<u32>,
                      task_status_id: u32, progress_rate: u32) -> Result<String, String> {
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

    println!("title:{:?}", title);
    println!("assignee_resource_id:{:?}", assignee_resource_id);
    println!("task_status_id:{:?}", task_status_id);
    println!("progress_rate:{:?}", progress_rate);
    // TODO 追加実行
    Ok("成功だよTODO".to_string())
}
