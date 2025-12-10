// pure.rs
// PURE FUNCTIONS → tidak boleh ada IO, tidak ada mut, tidak ada side-effects.

use crate::BASE_URL;
use crate::JobStatusResponse;

pub fn calc_reduction_percent(original_size: u64, compressed_size: u64) -> f64 {
    if original_size == 0 {
        0.0
    } else {
        (1.0 - (compressed_size as f64 / original_size as f64)) * 100.0
    }
}

pub fn build_download_url(file_name: &str) -> String {
    format!("{}/download/{}", BASE_URL, file_name)
}

pub fn build_job_status(
    job_id: String,
    original_filename: String,
    original_size: u64,
    compressed_size: u64,
    processing_time: f64,
    compressed_file_name: String,
) -> JobStatusResponse {
    let reduction_percent = calc_reduction_percent(original_size, compressed_size);
    let download_url = build_download_url(&compressed_file_name);

    JobStatusResponse {
        job_id,
        status: "done".to_string(),
        original_filename,
        original_size,
        compressed_size,
        reduction_percent,
        processing_time,
        download_url,
    }
}
