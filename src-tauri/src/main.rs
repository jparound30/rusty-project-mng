// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use sqlx::{Acquire, Connection, Error, Executor};
use tauri::State;
use crate::db_connection::db_connection::DbConnection;
use crate::models::users::{User, show_all};
use crate::utils::hash::hash;

mod db_connection;
mod utils;
mod models;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn greet(connection: State<'_, DbConnection>, name: &str) -> Result<String, String> {

    let transactionResult =  connection.pool.begin().await;
    if transactionResult.is_err() {
        return Err(transactionResult.err().unwrap().to_string())
    }

    let mut transaction = transactionResult.unwrap();
    let connResult = transaction.acquire().await;
    if connResult.is_err() {
        return Err(connResult.err().unwrap().to_string())
    }

    let conn = connResult.ok().unwrap();
    let _ = create_table(conn);
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

#[derive(Debug, Serialize, Deserialize)]
struct UserMsg {
    user_id: u32,
    username: String,
}
#[tauri::command]
async fn authenticate(connection: State<'_, DbConnection>, username: &str, password: &str) -> Result<UserMsg, String> {

    let transactionResult =  connection.pool.begin().await;
    if transactionResult.is_err() {
        return Err(transactionResult.err().unwrap().to_string())
    }

    let mut transaction = transactionResult.unwrap();
    let connResult = transaction.acquire().await;
    if connResult.is_err() {
        return Err(connResult.err().unwrap().to_string())
    }

    let conn = connResult.ok().unwrap();

    let result_auth = models::authentications::authenticate(conn, username, password).await;

    match result_auth {
        Ok(isAuth) => {
            if isAuth {
                let user = User::get(conn, username.to_string()).await.unwrap().unwrap();
                Ok(UserMsg {
                    user_id: user.user_id,
                    username: user.username,
                })
            } else {
                Err(format!("{}", "認証エラー:ユーザ名パスワード誤り".to_string()))
            }
        }
        Err(err) => {
            Err(format!("{}{}", "認証エラー:".to_string(), err.to_string()))
        }
    }
}

pub async fn create_table(conn:  &mut sqlx::SqliteConnection) -> Result<(), Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
                user_id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT NOT NULL,
                password_hash TEXT,
                salt TEXT
            )",
        ).await?;
    Ok(())
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
    {
        let mut poo_conn = db_connection.pool.acquire().await.expect("pool fail");
        let conn = poo_conn.acquire().await.expect("pool.acquire");
        let _ = create_table(conn).await;
        // user: user, password: pass でユーザを追加しておく（テスト用）
        let password_hash = hash("pass".to_string());
        println!("password_hash: [{:?}]", password_hash);
        let test_result = User::add(conn, "user".to_string(), password_hash.0, password_hash.1).await;
    }

    tauri::Builder::default()
        .manage(db_connection)
        .invoke_handler(tauri::generate_handler![greet, authenticate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
