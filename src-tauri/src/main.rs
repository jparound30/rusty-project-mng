// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sqlx::Acquire;
use tauri::{Manager, Menu, State};
use tauri::async_runtime::block_on;

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

    let menu = Menu::new(); // configure the menu

    let app = tauri::Builder::default()
        .menu(menu)
        .on_menu_event(|event| {
            match event.menu_item_id() {
                "quit" => {
                    println!("menu:quit");
                    std::process::exit(0);
                }
                "close" => {
                    println!("menu:close");
                    event.window().close().unwrap();
                }
                t => {
                    println!("menu: {:?}", t);
                }
            }
        })
        .manage(db_connection)
        .invoke_handler(tauri::generate_handler![
            greet,
            crate::apis::authentication::authenticate,
            crate::apis::tasks::task_add,
            crate::apis::task_status::task_status_list,
            crate::apis::resources::resources_list,
            crate::apis::tasks::task_all_full,
            crate::apis::tasks::task_simple_all,
        ])
        .build(tauri::generate_context!())
        // .run(tauri::generate_context!())
        .expect("error while running tauri application");

    app.run(|_app_handle, event| match event {
        #[allow(unused_variables)]
        tauri::RunEvent::ExitRequested { api, .. } => {
            println!("on_window_event:Destroyed");
            let db_connection = _app_handle.state::<DbConnection>();
            tokio::task::block_in_place(|| {
                block_on(async {
                    let _ = db_connection.pool.close().await;
                    println!("db closed.");
                })
            });
        }
        _ => {}
    });
}
