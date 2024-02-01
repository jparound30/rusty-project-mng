use sqlx::Acquire;
use tauri::State;

use crate::db_connection::db_connection::DbConnection;
use crate::models;
use crate::models::resources::Resources;

#[tauri::command]
pub async fn resources_list(connection: State<'_, DbConnection>) -> Result<Vec<Resources>, String> {

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

    let resources_list = models::resources::Resources::all(conn).await;

    match resources_list {
        Ok(list) => {Ok(list)}
        Err(err) => {Err(err.to_string())}
    }
}

