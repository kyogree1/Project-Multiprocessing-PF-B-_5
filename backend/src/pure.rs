    // pure.rs
    // PURE FUNCTIONS → tidak boleh ada IO, tidak ada mut, tidak ada side-effects.

    use crate::BASE_URL;
    use crate::JobStatusResponse;

    /// Hitung persentase reduksi ukuran file dengan clean pattern matching.
    pub fn calc_reduction_percent(original_size: u64, compressed_size: u64) -> f64 {
        match original_size {
            0 => 0.0,
            size => {
                let ratio = compressed_size as f64 / size as f64;
                (1.0 - ratio) * 100.0
            }
        }
    }

    /// Bangun URL download tanpa concatenation yang berantakan.
    pub fn build_download_url(file_name: &str) -> String {
        format!("{}/download/{}", BASE_URL, file_name)
    }

    /// Builder untuk status job: pure, simple, clean.
    pub fn build_job_status(
        job_id: String,
        original_filename: String,
        original_size: u64,
        compressed_size: u64,
        processing_time: f64,
        compressed_file_name: String,
    ) -> JobStatusResponse {
        JobStatusResponse {
            job_id,
            status: "done".to_string(),
            original_filename,
            original_size,
            compressed_size,
            reduction_percent: calc_reduction_percent(original_size, compressed_size),
            processing_time,
            download_url: build_download_url(&compressed_file_name),
        }
    }
