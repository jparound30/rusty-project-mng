pub mod users {
    use rusqlite::{Connection, OptionalExtension, Result};

    #[derive(Debug)]
    pub struct User {
        user_id: i32,
        username: String,
    }

    #[derive(Debug)]
    pub struct UserAuthentication {
        password_hash: String,
        salt: String,
    }

    impl User {
        pub fn add(conn: &Connection, username: String, password_hash: String, salt: String) -> Result<User>  {
            let mut new_user = User {
                user_id: 0,
                username: username,
            };
            let inserted_row_cnt = conn.execute(
                "INSERT INTO users \
                    (username, password_hash, salt)\
                 VALUES\
                    (?1,?2, ?3)",
                (&new_user.username, password_hash, salt))?;
            println!("insert return: {:?}", inserted_row_cnt);

            new_user.user_id = conn.last_insert_rowid() as i32;
            Ok(new_user)
        }

        pub fn get(conn: &Connection, username: String) -> Result<Option<User>> {
            let mut stmt = conn.prepare("\
                        SELECT user_id, username, password_hash, salt \
                        FROM users \
                        WHERE username = ?1
                        ")?;
            let ret = stmt.query_row([username], |row| {
                Ok(
                    User {
                    user_id: row.get(0)?,
                    username: row.get(1)?,
                    }
                )
            }).optional();
            ret
        }
    }

    pub fn create_table(connection: &Connection) -> Result<()> {
        connection.execute(
            "CREATE TABLE IF NOT EXISTS users (
                user_id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT NOT NULL,
                password_hash TEXT,
                salt TEXT
            )",
            (), )?;
        Ok(())
    }


    #[allow(dead_code)]
    pub fn show_all(connection: &Connection) -> Result<()> {
        let mut stmt = connection.prepare("SELECT user_id, username, password_hash, salt FROM users")?;
        let user_iter = stmt.query_map([], |row| {
            Ok(
                (
                    User {
                        user_id: row.get(0)?,
                        username: row.get(1)?,
                    },
                    UserAuthentication {
                        password_hash: row.get(2)?,
                        salt: row.get(3)?,
                    }
                )
            )
        })?;

        for user in user_iter {
            let (u, ua) = user.unwrap();
            println!("Found user {:?}, {:?}", u, ua);
        }
        Ok(())
    }

}
