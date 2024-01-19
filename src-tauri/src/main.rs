// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::ops::Deref;
use tauri::State;
use crate::db_connection::db_connection::DbConnection;
use crate::users::users::{create_table, show_all};

mod users;
mod db_connection;

use crate::users::users::User;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(connection: State<DbConnection>, name: &str) -> String {
    // let conn = Connection::open_in_memory().unwrap();
    let mutex_conn = connection.connection.lock().unwrap();
    let conn = mutex_conn.as_ref().unwrap();
    println!("{}", conn.is_autocommit());

    let _ = create_table(conn);
    let test_result = User::add_new_user(conn, name.to_string(), name.to_string());

    // let test_result = db_test(conn, name);

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

fn main() {
    tauri::Builder::default()
        .manage(DbConnection::create())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
