use rusqlite::{params, Result};

use crate::db::connection::get_connection;

#[derive(Debug)]
pub struct Job {
    pub id: i64,
    pub original_filename: String,
    pub original_path: String,
    pub compressed_path: Option<String>,
    pub status: String,

    pub original_size: i64,
    pub compressed_size: i64,
    pub processing_time: f64,
}

pub fn create_job(filename: &str, path: &str) -> Result<i64> {
    let conn = get_connection()?;

    conn.execute(
        "INSERT INTO jobs (original_filename, original_path, status)
         VALUES (?1, ?2, 'pending')",
        params![filename, path],
    )?;

    Ok(conn.last_insert_rowid())
}

pub fn fetch_next_pending_job() -> Result<Option<Job>> {
    let conn = get_connection()?;

    let mut stmt = conn.prepare(
        "SELECT 
            id,
            original_filename,
            original_path,
            compressed_path,
            status,
            original_size,
            compressed_size,
            processing_time
         FROM jobs
         WHERE status = 'pending'
         ORDER BY id ASC
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
            original_size: row.get(5)?,
            compressed_size: row.get(6)?,
            processing_time: row.get(7)?,
        }))
    } else {
        Ok(None)
    }
}

pub fn update_job_status(job_id: i64, new_status: &str) -> Result<()> {
    let conn = get_connection()?;

    conn.execute(
        "UPDATE jobs SET status = ?1 WHERE id = ?2",
        params![new_status, job_id],
    )?;

    Ok(())
}

pub fn update_compressed_path(job_id: i64, path: &str) -> Result<()> {
    let conn = get_connection()?;

    conn.execute(
        "UPDATE jobs SET compressed_path = ?1 WHERE id = ?2",
        params![path, job_id],
    )?;

    Ok(())
}

pub fn get_job_by_id(job_id: i64) -> Result<Option<Job>> {
    let conn = get_connection()?;

    let mut stmt = conn.prepare(
        "SELECT 
            id,
            original_filename,
            original_path,
            compressed_path,
            status,
            original_size,
            compressed_size,
            processing_time
         FROM jobs
         WHERE id = ?1",
    )?;

    let mut rows = stmt.query(params![job_id])?;

    if let Some(row) = rows.next()? {
        Ok(Some(Job {
            id: row.get(0)?,
            original_filename: row.get(1)?,
            original_path: row.get(2)?,
            compressed_path: row.get(3)?,
            status: row.get(4)?,
            original_size: row.get(5)?,
            compressed_size: row.get(6)?,
            processing_time: row.get(7)?,
        }))
    } else {
        Ok(None)
    }
}

pub fn update_job_metrics(job_id: i64, original: i64, compressed: i64, time: f64) -> Result<()> {
    let conn = get_connection()?;

    conn.execute(
        "UPDATE jobs
         SET original_size = ?1,
             compressed_size = ?2,
             processing_time = ?3
         WHERE id = ?4",
        params![original, compressed, time, job_id],
    )?;

    Ok(())
}

