// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use tauri::State;
use crate::db_connection::db_connection::DbConnection;
use crate::models::users::{User, create_table, show_all};
use crate::utils::hash::hash;

mod db_connection;
mod utils;
mod models;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(connection: State<DbConnection>, name: &str) -> String {

    let mutex_conn = connection.connection.lock().unwrap();
    let conn = mutex_conn.as_ref().unwrap();
    println!("{}", conn.is_autocommit());

    let _ = create_table(conn);
    let password_hash = hash(name.to_string());
    println!("password_hash: [{:?}]", password_hash);
    let test_result = User::add(conn, name.to_string(), password_hash.0, password_hash.1);

    let _ = show_all(conn);

    match test_result {
        Ok(user) => {
            format!("Hello, {:?}! You've been greeted from Rust!", user)
        }
        Err(_) => {
            "error".to_string()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct UserMsg {
    user_id: u32,
    username: String,
}
#[tauri::command]
fn authenticate(connection: State<DbConnection>, username: &str, password: &str) -> Result<UserMsg, String> {

    let mutex_conn = connection.connection.lock().unwrap();
    let conn = mutex_conn.as_ref().unwrap();
    println!("{}", conn.is_autocommit());

    let result_auth = models::authentications::authenticate(conn, username, password);

    if !result_auth {
        Err("認証エラー".to_string())
    } else {
        let user = User::get(conn, username.to_string()).unwrap().unwrap();
        Ok(UserMsg {
            user_id: user.user_id,
            username: user.username,
        })
    }
}

fn main() {
    fn show_dir(relative_path: &str) {
        // Get current directory
        let current_dir = std::env::current_dir().expect("Failed to get current directory");
        let path = current_dir.join(relative_path);

        println!("absolute path for [{}] is [{}]", relative_path, path.display())
    }
    show_dir(".env");

    tauri::Builder::default()
        .manage(DbConnection::create())
        .invoke_handler(tauri::generate_handler![greet, authenticate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
