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
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: compress_worker <input_path> <output_path>");
        process::exit(1);
    }

    let input = PathBuf::from(&args[1]);
    let output = PathBuf::from(&args[2]);

    if let Err(e) = compress_pdf_high(&input, &output) {
        eprintln!("Compression failed: {e}");
        process::exit(1);
    }
}
