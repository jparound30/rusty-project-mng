pub mod db_connection {
    use std::sync::Mutex;
    use rusqlite::Connection;

    pub struct DbConnection {
        pub connection: Mutex<Option<Connection>>,
    }

    impl DbConnection {
        pub fn create() -> Self {
            let conn = Connection::open_in_memory().unwrap();
            DbConnection {
                connection: Mutex::new(Some(conn))
            }
        }
    }
}