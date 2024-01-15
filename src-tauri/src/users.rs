pub mod users {
    use rusqlite::{Connection, Result};

    #[derive(Debug)]
    struct User {
        user_id: i32,
        username: String,
        password_hash: String,
    }

    pub fn create_table(connection: &Connection) -> Result<()> {
        connection.execute(
            "CREATE TABLE IF NOT EXISTS users (
                user_id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT NOT NULL,
                password_hash TEXT
            )",
            (), )?;
        Ok(())
    }

    pub fn db_test(connection: &Connection, name: &str) -> Result<()> {
        let me = User {
            user_id: 0,
            username: name.to_string(),
            password_hash: "password".to_string(),
        };
        connection.execute("INSERT INTO users (username, password_hash) VALUES (?1,?2)",
                           (&me.username, &me.password_hash),
        )?;

        let mut stmt = connection.prepare("SELECT user_id, username, password_hash FROM users")?;
        let user_iter = stmt.query_map([], |row| {
            Ok(User {
                user_id: row.get(0)?,
                username: row.get(1)?,
                password_hash: row.get(2)?,
            })
        })?;

        for user in user_iter {
            println!("Found user {:?}", user.unwrap());
        }
        Ok(())
    }
}
