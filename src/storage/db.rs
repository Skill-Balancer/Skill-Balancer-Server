use rusqlite::Connection;

use crate::env::db_url;

pub fn create_database() -> Result<(), rusqlite::Error> {
    let conn = Connection::open(db_url())?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS saves (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            data BLOB NOT NULL,
            created_at TEXT NOT NULL
        )",
        [],
    )?;
    Ok(())
}
