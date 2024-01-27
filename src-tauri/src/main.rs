// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sqlx::Acquire;
use tauri::State;

use crate::db_connection::db_connection::DbConnection;
use crate::models::users::{show_all, User};
use crate::utils::hash::hash;

mod db_connection;
mod utils;
mod models;
mod apis;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn greet(connection: State<'_, DbConnection>, name: &str) -> Result<String, String> {

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
    let password_hash = hash(name.to_string());
    println!("password_hash: [{:?}]", password_hash);
    let test_result = User::add(conn, name.to_string(), password_hash.0, password_hash.1).await;

    let _ = show_all(conn);

    match test_result {
        Ok(user) => {
            Ok(format!("Hello, {:?}! You've been greeted from Rust!", user))
        }
        Err(_) => {
            Err("追加エラー".to_string())
        }
    }
}

#[tokio::main]
async fn main() {
    fn show_dir(relative_path: &str) {
        // Get current directory
        let current_dir = std::env::current_dir().expect("Failed to get current directory");
        let path = current_dir.join(relative_path);

        println!("absolute path for [{}] is [{}]", relative_path, path.display())
    }
    show_dir(".env");

    let db_connection = DbConnection::create().await;

    // DBマイグレーション?
    // sqlx::migrate!().run(&db_connection.pool).await?;

    tauri::Builder::default()
        .manage(db_connection)
        .invoke_handler(tauri::generate_handler![
            greet,
            crate::apis::authentication::authenticate,
            crate::apis::tasks::task_add,
            crate::apis::task_status::task_status_list
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
