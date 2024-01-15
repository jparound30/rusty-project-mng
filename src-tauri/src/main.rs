// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rusqlite::Connection;
use tauri::State;
use crate::db_connection::db_connection::DbConnection;
use crate::users::users::{create_table, db_test};

mod users;
mod db_connection;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(connection: State<DbConnection>, name: &str) -> String {
    // let conn = Connection::open_in_memory().unwrap();
    let mutexedConn = connection.connection.lock().unwrap();
    let conn = mutexedConn.as_ref().unwrap();
    println!("{}", conn.is_autocommit());
    let _ = create_table(conn);
    let test_result = db_test(conn, name);
    match test_result {
        Ok(_) => {
            format!("Hello, {}! You've been greeted from Rust!", name)
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
