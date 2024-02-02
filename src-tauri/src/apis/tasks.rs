use sqlx::Acquire;
use tauri::State;
use crate::db_connection::db_connection::DbConnection;
use crate::models;
use crate::models::tasks::{Task, TaskSimple};

#[tauri::command]
pub async fn task_add(connection: State<'_, DbConnection>, title: &str, description: Option<&str>, assignee_resource_id: Option<i64>,
                      parent_task_id: Option<i64>, start_date: Option<String>, due_date: Option<String>,
                      estimated_time: Option<i64>, actual_time: Option<i64>, planned_value: Option<i64>,
                      task_status_id: i64, progress_rate: i64) -> Result<String, String> {
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

    let new_task = Task {
        task_id: 0,
        title: title.to_string(),
        description: if description.is_none() { None } else {Some(description.unwrap().to_string())},
        assignee_resource_id: assignee_resource_id,
        parent_task_id: parent_task_id,
        start_date: start_date,
        due_date: due_date,
        estimated_time: estimated_time,
        actual_time: actual_time,
        planed_value: planned_value,
        task_status_id: task_status_id,
        progress_rate: progress_rate,
    };

    let result_task_add = new_task.add(conn).await;
    match result_task_add {
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
pub async fn task_all(connection: State<'_, DbConnection>) -> Result<Vec<Task>, String> {

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

    let task_list = models::tasks::Task::all(conn).await;

    match task_list {
        Ok(list) => {Ok(list)}
        Err(err) => {Err(err.to_string())}
    }
}

#[tauri::command]
pub async fn task_simple_all(connection: State<'_, DbConnection>) -> Result<Vec<TaskSimple>, String> {

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

    let task_simple_list = models::tasks::Task::all_with_id_and_title(conn).await;

    match task_simple_list {
        Ok(list) => {Ok(list)}
        Err(err) => {Err(err.to_string())}
    }
}