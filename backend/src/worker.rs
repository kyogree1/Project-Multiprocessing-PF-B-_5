use std::{
    fs,
    path::{Path, PathBuf},
    time::Instant,
    thread,
    time::Duration,
    env,
};

mod db;
mod gs_compressor;

use db::jobs::{
    fetch_next_pending_job,
    update_job_status,
    update_compressed_path,
    update_job_metrics,
};

use gs_compressor::compress_pdf_high;

const COMPRESSED_DIR: &str = "data/compressed";

fn main() {
    let id = env::var("WORKER_ID").unwrap_or("X".into());
    println!("[WORKER-{id}] Worker process aktif");

    fs::create_dir_all(COMPRESSED_DIR).ok();

    loop {
        match fetch_next_pending_job() {
            Ok(Some(job)) => {
                println!("[WORKER-{id}] Memproses job {}...", job.id);

                update_job_status(job.id, "processing").ok();

                let start = Instant::now();
                let input = Path::new(&job.original_path);

                if !input.exists() {
                    eprintln!("[WORKER-{id}] File tidak ditemukan");
                    update_job_status(job.id, "error").ok();
                    continue;
                }

                let output_name = format!("{}_{}", job.id, job.original_filename);
                let output: PathBuf = PathBuf::from(COMPRESSED_DIR).join(&output_name);

                match compress_pdf_high(input, &output) {
                    Ok(_) => {
                        let elapsed = start.elapsed();

                        let original_size = fs::metadata(input).map(|m| m.len() as i64).unwrap_or(0);
                        let compressed_size = fs::metadata(&output).map(|m| m.len() as i64).unwrap_or(0);

                        update_compressed_path(job.id, output.to_string_lossy().as_ref()).ok();
                        update_job_metrics(job.id, original_size, compressed_size, elapsed.as_secs_f64()).ok();
                        update_job_status(job.id, "done").ok();

                        println!(
                            "[WORKER-{id}] Job {} selesai ({} ms).",
                            job.id,
                            elapsed.as_millis()
                        );
                    }
                    Err(e) => {
                        eprintln!("[WORKER-{id}] ERROR GS: {}", e);
                        update_job_status(job.id, "error").ok();
                    }
                }
            }
            Ok(None) => {
                thread::sleep(Duration::from_millis(250));
            }
            Err(e) => {
                eprintln!("[WORKER-{id}] ERROR ambil job: {:?}", e);
                thread::sleep(Duration::from_secs(1));
            }
        }
    }
}
