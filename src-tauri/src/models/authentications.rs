use sqlx::{Error, Executor};

use crate::utils::hash::verify;

#[derive(Debug, sqlx::FromRow)]
struct UserAuthentication {
    user_id: i32,
    username: String,
    password_hash: String,
    salt: String,
}

pub async fn authenticate(conn: &mut sqlx::SqliteConnection, username: &str, password: &str) -> Result<bool, Error> {
    let query_result = sqlx::query_as::<_, UserAuthentication>("\
                        SELECT user_id, username, password_hash, salt \
                        FROM users \
                        WHERE username = ?1
                        ")
        .bind(username)
        .fetch_optional(conn).await;
    if query_result.is_err() {
        return Err(query_result.err().unwrap());
    }

    let userOption = query_result.unwrap();
    if userOption.is_none() {
        return Ok(false);
    }

    let user_authentication = userOption.unwrap();
    Ok(verify(password.to_string(), user_authentication.password_hash))
}
