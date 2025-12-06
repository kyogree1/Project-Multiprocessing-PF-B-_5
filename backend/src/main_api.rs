use std::{fs, net::SocketAddr, path::PathBuf, time::Instant};

use axum::{
    extract::{DefaultBodyLimit, Multipart, Path as AxumPath},
    http::{header, HeaderValue, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use rayon::prelude::*;
use serde::Serialize;
use tokio::{net::TcpListener, task::spawn_blocking};
use tower_http::cors::{Any, CorsLayer};
use uuid::Uuid;

// === modul kompresor GhostScript ===
mod gs_compressor;
use crate::gs_compressor::compress_pdf_high;

const UPLOAD_DIR: &str = "data/uploads";
const COMPRESSED_DIR: &str = "data/compressed";
const BASE_URL: &str = "http://localhost:3000";

#[derive(Serialize)]
struct JobStatusResponse {
    #[serde(rename = "jobId")]
    job_id: String,

    #[serde(rename = "status")]
    status: String,

    #[serde(rename = "originalFilename")]
    original_filename: String,

    #[serde(rename = "originalSize")]
    original_size: u64,

    #[serde(rename = "compressedSize")]
    compressed_size: u64,

    #[serde(rename = "reductionPercent")]
    reduction_percent: f64,

    #[serde(rename = "processingTime")]
    processing_time: f64,

    #[serde(rename = "downloadUrl")]
    download_url: String,
}

#[derive(Clone)]
struct PendingUpload {
    stored_input_name: String,
    original_filename: String,
    file_bytes: Vec<u8>,
}

#[tokio::main]
async fn main() {
    fs::create_dir_all(UPLOAD_DIR).expect("failed to create upload dir");
    fs::create_dir_all(COMPRESSED_DIR).expect("failed to create compressed dir");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        // 🔹 Fitur multithreading (Rayon)
        .route("/compress/rayon", post(handle_compress_rayon))
        // 🔹 Fitur single-thread (sequential)
        .route("/compress/single", post(handle_compress_single))
        // (opsional) alias lama /compress → mode Rayon
        .route("/compress", post(handle_compress_rayon))
        // Download hasil kompresi
        .route("/download/:file", get(handle_download))
        // Besarkan limit upload, misal 50MB
        .layer(DefaultBodyLimit::disable())
        .layer(cors);

    let addr: SocketAddr = "0.0.0.0:3000"
        .parse()
        .expect("invalid bind address");

    println!("API running at {}", BASE_URL);
    println!("  • POST /compress        (multithreading / Rayon)");
    println!("  • POST /compress/rayon  (multithreading / Rayon)");
    println!("  • POST /compress/single (single-thread)");
    println!("  • GET  /download/:file");

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// ======================== HANDLER ========================

async fn handle_compress_rayon(
    multipart: Multipart,
) -> Result<Json<Vec<JobStatusResponse>>, (StatusCode, String)> {
    let uploads = collect_uploads(multipart).await?;
    let results = spawn_blocking(move || process_uploads_parallel(uploads))
        .await
        .map_err(|e| internal_error(format!("Join error: {e}")))?
        .map_err(internal_error)?;
    Ok(Json(results))
}

async fn handle_compress_single(
    multipart: Multipart,
) -> Result<Json<Vec<JobStatusResponse>>, (StatusCode, String)> {
    let uploads = collect_uploads(multipart).await?;
    let results = spawn_blocking(move || process_uploads_sequential(uploads))
        .await
        .map_err(|e| internal_error(format!("Join error: {e}")))?
        .map_err(internal_error)?;
    Ok(Json(results))
}

async fn collect_uploads(
    mut multipart: Multipart,
) -> Result<Vec<PendingUpload>, (StatusCode, String)> {
    let mut uploads: Vec<PendingUpload> = Vec::new();

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(internal_error)?
    {
        let name = field.file_name().map(|s| s.to_string());
        let data = field.bytes().await.map_err(internal_error)?;

        let original_filename = name.unwrap_or_else(|| "upload.pdf".to_string());
        let id = Uuid::new_v4().to_string();
        let stored_input_name = format!("{id}-{original_filename}");

        uploads.push(PendingUpload {
            stored_input_name,
            original_filename,
            file_bytes: data.to_vec(),
        });
    }

    if uploads.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "No file uploaded".to_string(),
        ));
    }

    Ok(uploads)
}

// ==================== PROSES PARALLEL (RAYON) ====================

fn process_uploads_parallel(
    uploads: Vec<PendingUpload>,
) -> Result<Vec<JobStatusResponse>, String> {
    let upload_dir = UPLOAD_DIR.to_string();
    let compressed_dir = COMPRESSED_DIR.to_string();

    uploads
        .into_par_iter()
        .map(|upload| process_single_upload(upload, &upload_dir, &compressed_dir))
        .collect()
}

// ==================== PROSES SEQUENTIAL ====================

fn process_uploads_sequential(
    uploads: Vec<PendingUpload>,
) -> Result<Vec<JobStatusResponse>, String> {
    let upload_dir = UPLOAD_DIR.to_string();
    let compressed_dir = COMPRESSED_DIR.to_string();

    uploads
        .into_iter()
        .map(|upload| process_single_upload(upload, &upload_dir, &compressed_dir))
        .collect()
}

// ==================== LOGIKA KOMPREESI SATU FILE ====================

fn process_single_upload(
    upload: PendingUpload,
    upload_dir: &str,
    compressed_dir: &str,
) -> Result<JobStatusResponse, String> {
    let PendingUpload {
        stored_input_name,
        original_filename,
        file_bytes,
    } = upload;

    let job_id = Uuid::new_v4().to_string();

    let input_path = PathBuf::from(upload_dir).join(&stored_input_name);
    fs::write(&input_path, &file_bytes)
        .map_err(|e| format!("Gagal menulis file input: {e}"))?;

    let compressed_file_name = format!("compressed-{stored_input_name}");
    let output_path = PathBuf::from(compressed_dir).join(&compressed_file_name);

    let start = Instant::now();
    // 🔹 Pakai modul gs_compressor, bukan fungsi lokal
    compress_pdf_high(input_path.as_path(), output_path.as_path())
        .map_err(|e| format!("Gagal kompres PDF: {e}"))?;
    let elapsed = start.elapsed().as_secs_f64();

    let original_size = fs::metadata(&input_path)
        .map_err(|e| format!("Gagal baca metadata input: {e}"))?
        .len();
    let compressed_size = fs::metadata(&output_path)
        .map_err(|e| format!("Gagal baca metadata output: {e}"))?
        .len();

    let reduction_percent = if original_size == 0 {
        0.0
    } else {
        (1.0 - (compressed_size as f64 / original_size as f64)) * 100.0
    };

    let download_url = format!(
        "{}/download/{}",
        BASE_URL, compressed_file_name
    );

    Ok(JobStatusResponse {
        job_id,
        status: "done".to_string(),
        original_filename,
        original_size,
        compressed_size,
        reduction_percent,
        processing_time: elapsed,
        download_url,
    })
}

// ======================== DOWNLOAD ========================

async fn handle_download(
    AxumPath(file): AxumPath<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let path = PathBuf::from(COMPRESSED_DIR).join(&file);

    if !path.exists() {
        return Err((
            StatusCode::NOT_FOUND,
            "File not found".to_string(),
        ));
    }

    let bytes = fs::read(&path).map_err(internal_error)?;

    let dispo = format!(
        "attachment; filename=\"{}\"",
        path.file_name()
            .unwrap_or_default()
            .to_string_lossy()
    );

    let headers = [
        (header::CONTENT_TYPE, HeaderValue::from_static("application/pdf")),
        (
            header::CONTENT_DISPOSITION,
            HeaderValue::from_str(&dispo)
                .unwrap_or_else(|_| HeaderValue::from_static("attachment")),
        ),
    ];

    Ok((headers, bytes))
}

// ======================== ERROR HELPER ========================

fn internal_error<E: std::fmt::Display>(
    err: E,
) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
