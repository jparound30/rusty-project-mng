use rusqlite::Connection;

use crate::utils::hash::verify;

#[derive(Debug)]
struct UserAuthentication {
    user_id: i32,
    username: String,
    password_hash: String,
    salt: String,
}

pub fn authenticate(conn: &Connection, username: &str, password: &str) -> bool {
    let mut stmt = conn.prepare("\
                        SELECT user_id, username, password_hash, salt \
                        FROM users \
                        WHERE username = ?1
                        ");
    if stmt.is_err() {
        return false;
    }
    let ret = stmt.unwrap().query_row([username], |row| {
        Ok(
            UserAuthentication {
                user_id: row.get(0)?,
                username: row.get(1)?,
                password_hash: row.get(2)?,
                salt: row.get(3)?,
            }
        )
    });
    if ret.is_err() {
        return false;
    }

    let user_authentication = ret.unwrap();
    verify(password.to_string(), user_authentication.password_hash)
}
