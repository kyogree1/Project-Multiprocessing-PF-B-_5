// io_ops.rs
// Semua operasi I/O berat (tulis file, metadata, panggil worker, timing)

use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::Instant,
};

use crate::{PendingUpload, WORKER_BIN};

/// Melakukan semua operasi I/O untuk satu upload:
/// - tulis file ke disk
/// - panggil worker proses
/// - baca ukuran sebelum/sesudah
/// - mengembalikan (original, compressed, elapsed, output_path)
pub fn execute_file_operations(
    upload: &PendingUpload,
    upload_dir: &str,
    compressed_dir: &str,
) -> Result<(u64, u64, f64, PathBuf), String> {
    // Tulis file input ke disk (IO)
    let input_path = PathBuf::from(upload_dir).join(&upload.stored_input_name);
    fs::write(&input_path, &upload.file_bytes)
        .map_err(|e| format!("Gagal menulis file input: {e}"))?;

    // Tentukan nama & path file output terkompres
    let compressed_file_name = format!("compressed-{}", upload.stored_input_name);
    let output_path = PathBuf::from(compressed_dir).join(&compressed_file_name);

    // Baca ukuran awal sebelum kompres
    let original_size = fs::metadata(&input_path)
        .map_err(|e| format!("Gagal baca metadata input: {e}"))?
        .len();

    // Kompres pakai GhostScript (Pemanggilan Proses Eksternal)
    let start = Instant::now();
    run_worker_process(input_path.as_path(), output_path.as_path())
        .map_err(|e| format!("Gagal kompres PDF: {e}"))?;
    let elapsed = start.elapsed().as_secs_f64();

    // Baca ukuran sesudah kompres
    let compressed_size = fs::metadata(&output_path)
        .map_err(|e| format!("Gagal baca metadata output: {e}"))?
        .len();

    Ok((original_size, compressed_size, elapsed, output_path))
}

// Fungsi private, hanya dipakai modul ini
fn run_worker_process(input: &Path, output: &Path) -> Result<(), String> {
    let input_str = input
        .to_str()
        .ok_or("invalid input path".to_string())?;

    let output_str = output
        .to_str()
        .ok_or("invalid output path".to_string())?;

    Command::new(WORKER_BIN)
        .arg(input_str)
        .arg(output_str)
        .status()
        .map_err(|e| format!("Failed to spawn worker: {e}"))?
        .success()
        .then_some(())
        .ok_or_else(|| format!("Worker exited with non-zero status"))
}

