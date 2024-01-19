pub mod db_connection {
    use std::sync::Mutex;
    use rusqlite::{Connection, OpenFlags};

    pub struct DbConnection {
        pub connection: Mutex<Option<Connection>>,
    }

    impl DbConnection {
        pub fn create() -> Self {
            let open_flags = OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_MEMORY | OpenFlags::SQLITE_OPEN_SHARED_CACHE;
            let conn = Connection::open_in_memory_with_flags(open_flags).unwrap();
            DbConnection {
                connection: Mutex::new(Some(conn))
            }
        }
    }
}