//! Service responsible for managing database queries.

embed_migrations!();

use std::sync::{Mutex, MutexGuard};

use diesel::prelude::*;

#[derive(Debug, PartialEq, Serialize, Queryable)]
pub struct Player {
    pub id: i32,
    pub url_key: String,
    pub name: String
}

pub struct Database {
    /// Due to low anticipated load, just keep a single database connection
    /// instead of setting up an r2d2 connection pool for now
    conn: Mutex<SqliteConnection>
}

impl Database {
    pub fn connect(filename: &str) -> Database {
        let conn = SqliteConnection::establish(filename).expect("Opening Sqlite");

        // Initialize database
        embedded_migrations::run_with_output(&conn, &mut std::io::stdout()).expect("Running migrations");

        Database {
            conn: Mutex::new(conn)
        }
    }

    fn conn(&self) -> MutexGuard<SqliteConnection> {
        self.conn.lock().unwrap()
    }

    pub fn player_from_key(&self, key: &str) -> QueryResult<Option<Player>> {
        use crate::schema::player::dsl::*;
        player.filter(urlKey.eq(key))
            .get_result(&*self.conn()).optional()
    }
}

