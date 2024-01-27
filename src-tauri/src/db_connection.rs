pub mod db_connection {
    use sqlx::{Pool, Sqlite};
    use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};

    pub struct DbConnection {
        pub pool: Pool<Sqlite>,
    }

    impl DbConnection {
        pub async fn create() -> Self {
            let options = SqliteConnectOptions::new()
                .filename("sample.db") // TODO どっかから持ってくる
                .create_if_missing(true)    // なければDBファイルを生成
                .pragma("foreign_keys", "true");  // 外部キー制約有効化
            let pool = SqlitePoolOptions::new()
                .max_connections(5)
                .connect_with(options).await.expect("接続不可");
            DbConnection {
                pool
            }
        }
    }
}