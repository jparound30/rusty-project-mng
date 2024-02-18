use sqlx::Error;

use crate::utils::hash::verify;

#[allow(dead_code)]
#[derive(Debug, sqlx::FromRow)]
struct UserAuthentication {
    user_id: i64,
    username: String,
    password_hash: Option<String>,
    salt: Option<String>,
}

pub async fn authenticate(conn: &mut sqlx::SqliteConnection, username: &str, password: &str) -> Result<bool, Error> {
    let query_result = sqlx::query_file_as!(UserAuthentication, "sqls/authentication/authenticate.sql", username)
        .fetch_optional(conn).await;
    if query_result.is_err() {
        return Err(query_result.err().unwrap());
    }

    let user_option = query_result.unwrap();
    if user_option.is_none() {
        return Ok(false);
    }

    let user_authentication = user_option.unwrap();

    match user_authentication.password_hash {
        Some(password_hash) => {
            Ok(verify(password.to_string(), password_hash))
        }
        None => {
            Ok(false)
        }
    }
}
