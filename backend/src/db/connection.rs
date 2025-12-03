use rusqlite::{Connection, Result};
use std::{fs, path::Path};

/// Path database = backend/data/jobs.db
const DB_PATH: &str = "data/jobs.db";

pub fn get_connection() -> Result<Connection> {
    let db_path = Path::new(DB_PATH);

    // Buat folder data/ kalau belum ada
    if let Some(parent) = db_path.parent() {
        fs::create_dir_all(parent).ok();
    }

    // Buka atau buat file database
    let conn = Connection::open(db_path)?;

    // Buat tabel jobs jika belum ada (field lengkap)
    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS jobs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            original_filename TEXT NOT NULL,
            original_path TEXT NOT NULL,
            compressed_path TEXT,
            status TEXT NOT NULL,
            original_size INTEGER DEFAULT 0,
            compressed_size INTEGER DEFAULT 0,
            processing_time REAL DEFAULT 0,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP
        );
        ",
        [],
    )?;

    Ok(conn)
}
