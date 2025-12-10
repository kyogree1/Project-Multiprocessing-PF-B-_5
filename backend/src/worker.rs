use std::{env, path::PathBuf, process};

mod gs_compressor;
use crate::gs_compressor::compress_pdf_high;

/// Binary *worker* mandiri untuk kebutuhan multiprocessing.
/// Proses ini tidak dijalankan langsung oleh user,
/// tapi dipanggil oleh `main_api` lewat:
///   `Command::new(WORKER_BIN).arg(input).arg(output)...`
///
/// Tugas worker:
/// 1. Menerima path input & output dari argumen CLI.
/// 2. Memanggil `compress_pdf_high` (yang pakai GhostScript).
/// 3. Keluar dengan exit code:
///      - 0  → kompresi sukses
///      - ≠0 → kompresi gagal (akan dibaca sebagai error oleh `main_api`).
fn main() {
    let mut args = env::args().skip(1);

    let input = args.next().unwrap_or_else(|| {
        eprintln!("Usage: compress_worker <input_path> <output_path>");
        process::exit(1);
    });

    let output = args.next().unwrap_or_else(|| {
        eprintln!("Usage: compress_worker <input_path> <output_path>");
        process::exit(1);
    });

    compress_pdf_high(&PathBuf::from(input), &PathBuf::from(output))
        .unwrap_or_else(|e| {
            eprintln!("Compression failed: {e}");
            process::exit(1);
        }); 
}
    