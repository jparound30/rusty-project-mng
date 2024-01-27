use sqlx::Error;

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub user_id: u32,
    pub username: String,
}

impl User {
    pub async fn add(conn: &mut sqlx::SqliteConnection, username: String, password_hash: String, salt: String) -> Result<User, Error> {
        let mut new_user = User {
            user_id: 0,
            username: username,
        };
        let last_insert_rowid = sqlx::query(
            "INSERT INTO users \
                    (username, password_hash, salt)\
                 VALUES\
                    (?1,?2, ?3)")
            .bind(&new_user.username)
            .bind(password_hash)
            .bind(salt)
            .execute(conn)
            .await?
            .last_insert_rowid();

        new_user.user_id = last_insert_rowid as u32;
        Ok(new_user)
    }

    pub async fn get(conn: &mut sqlx::SqliteConnection, username: String) -> Result<Option<User>, Error> {
        let option = sqlx::query_as::<_, User>("\
                        SELECT user_id, username \
                        FROM users \
                        WHERE username = ?1
                        ")
            .bind(username)
            .fetch_optional(conn)
            .await?;
        Ok(option)
    }
}

#[allow(dead_code)]
#[derive(Debug, sqlx::FromRow)]
struct UserAndAuth {
    user_id: i32,
    username: String,
    password_hash: String,
    salt: String,
}

#[allow(dead_code)]
pub async fn show_all(conn: &mut sqlx::SqliteConnection) -> Result<(), Error> {
    let users = sqlx::query_as::<_, UserAndAuth>("SELECT user_id, username, password_hash, salt FROM users")
        .fetch_all(conn)
        .await?;

    for user in users {
        println!("Found user {:?}", user);
    }
    Ok(())
}
