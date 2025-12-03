use rusqlite::{params, Result};
use crate::db::connection::get_connection;

pub fn add_log(job_id: i32, msg: &str) -> Result<()> {
    let conn = get_connection()?;

    conn.execute(
        "INSERT INTO worker_logs (job_id, message) VALUES (?, ?)",
        params![job_id, msg],
    )?;

    Ok(())
}
