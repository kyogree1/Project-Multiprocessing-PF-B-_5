use rusqlite::{Connection, Result};
use std::path::Path;
use std::fs;

/// Membuka koneksi ke database compressor.db
pub fn get_connection() -> Result<Connection> {
    // Path database: project/data/compressor.db
    let db_path = Path::new("../../data/compressor.db");

    // Pastikan folder data/ ada
    if let Some(parent) = db_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).expect("Gagal membuat folder data/");
        }
    }

    // Membuka (atau membuat) database
    let conn = Connection::open(db_path)?;

    Ok(conn)
}
