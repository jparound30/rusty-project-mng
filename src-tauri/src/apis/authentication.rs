use serde::{Deserialize, Serialize};
use sqlx::Acquire;
use tauri::State;
use crate::db_connection::db_connection::DbConnection;
use crate::models;
use crate::models::users::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserMsg {
    user_id: i64,
    username: String,
}
#[tauri::command]
pub async fn authenticate(connection: State<'_, DbConnection>, username: &str, password: &str) -> Result<UserMsg, String> {

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

    let result_auth = models::authentications::authenticate(conn, username, password).await;

    match result_auth {
        Ok(is_authenticated) => {
            if is_authenticated {
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

