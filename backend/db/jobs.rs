use rusqlite::{params, Result};
use chrono::{DateTime, Utc};

use crate::db::connection::get_connection;

#[derive(Debug)]
pub struct Job {
    pub id: i32,
    pub original_filename: String,
    pub original_path: String,
    pub compressed_path: Option<String>,
    pub status: String,
    pub created_at: String,
}

pub fn create_job(filename: &str, path: &str) -> Result<i64> {
    let conn = get_connection()?;

    conn.execute(
        "INSERT INTO jobs (original_filename, original_path) VALUES (?, ?)",
        params![filename, path],
    )?;

    Ok(conn.last_insert_rowid())
}

pub fn fetch_next_pending_job() -> Result<Option<Job>> {
    let conn = get_connection()?;

    let mut stmt = conn.prepare(
        "SELECT id, original_filename, original_path, compressed_path, status, created_at
         FROM jobs
         WHERE status = 'pending'
         ORDER BY created_at ASC
         LIMIT 1",
    )?;

    let mut rows = stmt.query([])?;

    if let Some(row) = rows.next()? {
        Ok(Some(Job {
            id: row.get(0)?,
            original_filename: row.get(1)?,
            original_path: row.get(2)?,
            compressed_path: row.get(3)?,
            status: row.get(4)?,
            created_at: row.get(5)?,
        }))
    } else {
        Ok(None)
    }
}

pub fn update_job_status(job_id: i32, new_status: &str) -> Result<()> {
    let conn = get_connection()?;

    conn.execute(
        "UPDATE jobs SET status = ? WHERE id = ?",
        params![new_status, job_id],
    )?;

    Ok(())
}

pub fn update_compressed_path(job_id: i32, path: &str) -> Result<()> {
    let conn = get_connection()?;

    conn.execute(
        "UPDATE jobs SET compressed_path = ? WHERE id = ?",
        params![path, job_id],
    )?;

    Ok(())
}
