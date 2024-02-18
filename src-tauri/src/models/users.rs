use sqlx::Error;

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub user_id: i64,
    pub username: String,
}

impl User {
    pub async fn add(conn: &mut sqlx::SqliteConnection, username: String, password_hash: String, salt: String) -> Result<User, Error> {
        let mut new_user = User {
            user_id: 0,
            username: "".to_string(),
        };
        let last_insert_rowid = sqlx::query_file!("sqls/users/add.sql",
            username, password_hash, salt,
        )
            .execute(conn)
            .await?
            .last_insert_rowid();

        new_user.user_id = last_insert_rowid;
        Ok(new_user)
    }

    pub async fn get(conn: &mut sqlx::SqliteConnection, username: String) -> Result<Option<User>, Error> {
        let option = sqlx::query_file_as!(User, "sqls/users/get.sql", username)
            .fetch_optional(conn)
            .await?;
        Ok(option)
    }
}

#[allow(dead_code)]
#[derive(Debug, sqlx::FromRow)]
struct UserAndAuth {
    user_id: i64,
    username: String,
    password_hash: Option<String>,
    salt: Option<String>,
}

#[allow(dead_code)]
pub async fn show_all(conn: &mut sqlx::SqliteConnection) -> Result<(), Error> {
    let users = sqlx::query_file_as!(UserAndAuth, "sqls/users/get_all.sql")
        .fetch_all(conn)
        .await?;

    for user in users {
        println!("Found user {:?}", user);
    }
    Ok(())
}
