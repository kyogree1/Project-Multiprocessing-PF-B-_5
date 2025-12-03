use std::{
    fs,
    net::SocketAddr,
    path::PathBuf,
};

use axum::{
    extract::{Multipart, Path as AxumPath},
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json,
    Router,
};
use serde::Serialize;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use uuid::Uuid;

use db::connection::get_connection;

mod db;

use db::jobs::{create_job, get_job_by_id};

const UPLOAD_DIR: &str = "data/uploads";
const COMPRESSED_DIR: &str = "data/compressed";
const BASE_URL: &str = "http://localhost:3000";
const FRONTEND_URL: &str = "http://localhost:5173";

#[derive(Serialize)]
struct CreateJobResponse {
    success: bool,
    #[serde(rename = "jobId")]
    job_id: i64,
    message: String,
}

#[derive(Serialize)]
struct JobStatusResponse {
    #[serde(rename = "jobId")]
    job_id: i64,
    status: String,
    #[serde(rename = "originalFilename")]
    original_filename: String,

    #[serde(rename = "originalSize")]
    original_size: u64,

    #[serde(rename = "compressedSize")]
    compressed_size: u64,

    #[serde(rename = "reductionPercent")]
    reduction: f64,

    #[serde(rename = "processingTime")]
    processing_time: f64,

    #[serde(rename = "downloadUrl")]
    download_url: Option<String>,
}


#[tokio::main]
async fn main() {
    // WAJIB!! Supaya tabel jobs dibuat
    let _ = get_connection().expect("Failed to init DB");

    fs::create_dir_all(UPLOAD_DIR).expect("failed to create upload dir");
    fs::create_dir_all(COMPRESSED_DIR).expect("failed to create compressed dir");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/compress", post(handle_compress))
        .route("/jobs/:id", get(handle_job_status))
        .route("/download/:file", get(handle_download))
        .layer(cors);

    let addr: SocketAddr = "0.0.0.0:3000"
        .parse()
        .expect("invalid bind address");

    println!("API running at {}", BASE_URL);
    println!("Frontend allowed at {}", FRONTEND_URL);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
// ---------------------- POST /compress -----------------------

async fn handle_compress(
    mut multipart: Multipart,
) -> Result<Json<CreateJobResponse>, (StatusCode, String)> {
    let mut file_name: Option<String> = None;
    let mut file_bytes: Option<Vec<u8>> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(internal_error)?
    {
        let name = field.file_name().map(|s| s.to_string());
        let data = field.bytes().await.map_err(internal_error)?;

        file_name = name.or(Some("upload.pdf".to_string()));
        file_bytes = Some(data.to_vec());
        break;
    }

    let file_bytes = match file_bytes {
        Some(b) => b,
        None => {
            return Err((
                StatusCode::BAD_REQUEST,
                "No file uploaded".to_string(),
            ))
        }
    };

    let original_filename =
        file_name.unwrap_or_else(|| "upload.pdf".to_string());

    let id = Uuid::new_v4().to_string();
    let stored_input_name = format!("{}-{}", id, &original_filename);

    // path RELATIF untuk disimpan ke DB & Worker
    let input_path = PathBuf::from(UPLOAD_DIR).join(&stored_input_name);

    fs::write(&input_path, &file_bytes).map_err(internal_error)?;

    let job_id = create_job(
        &stored_input_name,
        &input_path.to_string_lossy(),
    )
    .map_err(internal_error)?;

    Ok(Json(CreateJobResponse {
        success: true,
        job_id,
        message: "Job created, worker will process it".to_string(),
    }))
}

// ---------------------- GET /jobs/:id ------------------------

async fn handle_job_status(
    AxumPath(id): AxumPath<i64>,
) -> Result<Json<JobStatusResponse>, (StatusCode, String)> {
    let job = get_job_by_id(id).map_err(internal_error)?;

    let job = match job {
        Some(j) => j,
        None => {
            return Err((
                StatusCode::NOT_FOUND,
                "Job not found".to_string(),
            ))
        }
    };

    let download_url = job
        .compressed_path
        .as_ref()
        .map(|p| {
            let file_name = PathBuf::from(p)
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            format!("{}/download/{}", BASE_URL, file_name)
        });

    // RETURN JSON lengkap untuk frontend
    Ok(Json(JobStatusResponse {
        job_id: job.id,
        status: job.status,
        original_filename: job.original_filename,

        original_size: job.original_size as u64,
        compressed_size: job.compressed_size as u64,
        processing_time: job.processing_time,

        reduction: {
            if job.original_size == 0 {
                0.0
            } else {
                (1.0 - (job.compressed_size as f64 / job.original_size as f64)) * 100.0
            }
        },

        download_url,
    }))
}


// ---------------------- GET /download/:file ------------------

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

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/pdf"),
    );

    let dispo = format!("attachment; filename=\"{}\"", file);
    if let Ok(v) = HeaderValue::from_str(&dispo) {
        headers.insert(header::CONTENT_DISPOSITION, v);
    }

    Ok((headers, bytes))
}

// ---------------------- Helper error ------------------------

fn internal_error<E: std::fmt::Display>(
    err: E,
) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
