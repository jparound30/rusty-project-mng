#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tokio::main]
async fn main() {
    rusty_project_mng_lib::run().await;
}
