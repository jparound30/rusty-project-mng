use sqlx::Acquire;
use tauri::State;

use crate::db_connection::db_connection::DbConnection;
use crate::models;
use crate::models::resources::Resource;

#[tauri::command]
pub async fn resource_list(connection: State<'_, DbConnection>) -> Result<Vec<Resource>, String> {

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

    let resources_list = models::resources::Resource::all(conn).await;

    match resources_list {
        Ok(list) => {Ok(list)}
        Err(err) => {Err(err.to_string())}
    }
}

#[tauri::command]
pub async fn resource_get(connection: State<'_, DbConnection>, resource_id: i64) -> Result<Resource, String> {

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

    let resource = models::resources::Resource::get(conn, resource_id).await;

    match resource {
        Ok(info) => {
            match info {
                None => {
                    Err("Not Found".to_string())
                }
                Some(res) => {
                    Ok(res)
                }
            }
        }
        Err(err) => {Err(err.to_string())}
    }
}

#[tauri::command]
pub async fn resource_add(connection: State<'_, DbConnection>, name: String, cost_per_month: i64) -> Result<String, String> {
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

    let new_resource = Resource {
        resource_id: 0,
        name,
        cost_per_month,
    };

    let result_resource_add = new_resource.add(conn).await;
    match result_resource_add {
        Ok(_) => {
            let _ = transaction.commit().await;
            Ok("成功だよTODO".to_string())
        }
        Err(_) => {
            let _ = transaction.rollback().await;
            Ok("失敗だよTODO".to_string())
        }
    }
}

#[tauri::command]
pub async fn resource_update(connection: State<'_, DbConnection>,
                             resource_id: i64, name: String, cost_per_month: i64) -> Result<String, String> {
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

    let update_resource = Resource {
        resource_id,
        name,
        cost_per_month
    };

    let result = update_resource.update(conn).await;
    match result {
        Ok(_) => {
            let _ = transaction.commit().await;
            Ok("success".to_string())
        }
        Err(err) => {
            let _ = transaction.rollback().await;
            Err(err.to_string())
        }
    }
}
