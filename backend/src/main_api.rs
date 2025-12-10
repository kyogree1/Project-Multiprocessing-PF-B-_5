    use std::{fs, net::SocketAddr, path::PathBuf};

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

    mod pure;
    use crate::pure::build_job_status;

    mod io_ops;
    use crate::io_ops::execute_file_operations;

    // === KONSTANTA ===
    const UPLOAD_DIR: &str = "data/uploads";
    const COMPRESSED_DIR: &str = "data/compressed";
    const BASE_URL: &str = "http://localhost:3000";

    #[cfg(target_os = "windows")]
    const WORKER_BIN: &str = "target\\debug\\compress_worker.exe";
    #[cfg(not(target_os = "windows"))]
    const WORKER_BIN: &str = "target/debug/compress_worker";

    // === STRUCT DATA ===
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

    // ======================================================================
    // SETUP SERVER
    // ======================================================================

    fn setup_directories() {
        fs::create_dir_all(UPLOAD_DIR).expect("failed to create upload dir");
        fs::create_dir_all(COMPRESSED_DIR).expect("failed to create compressed dir");
    }

    fn create_app() -> Router {
        let cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any);

        Router::new()
            .route("/compress/rayon", post(handle_compress_rayon))
            .route("/compress/single", post(handle_compress_single))
            .route("/compress", post(handle_compress_rayon))
            .route("/download/:file", get(handle_download))
            .layer(DefaultBodyLimit::disable())
            .layer(cors)
    }

    async fn run_server(app: Router) {
        let addr: SocketAddr = "0.0.0.0:3000".parse().unwrap();
        println!("API running at {}", BASE_URL);

        let listener = TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }

    #[tokio::main]
    async fn main() {
        setup_directories();
        let app = create_app();
        run_server(app).await;
    }

    // ======================================================================
    // HANDLER
    // ======================================================================

    async fn execute_compression_job(
        uploads: Vec<PendingUpload>,
        processor: fn(Vec<PendingUpload>) -> Result<Vec<JobStatusResponse>, String>,
    ) -> Result<Json<Vec<JobStatusResponse>>, (StatusCode, String)> {
        let results = spawn_blocking(move || processor(uploads))
            .await
            .map_err(|e| internal_error(format!("Join error: {e}")))?
            .map_err(internal_error)?;

        Ok(Json(results))
    }

    async fn handle_compress_rayon(
        multipart: Multipart,
    ) -> Result<Json<Vec<JobStatusResponse>>, (StatusCode, String)> {
        let uploads = collect_uploads(multipart).await?;
        execute_compression_job(uploads, process_uploads_parallel).await
    }

    async fn handle_compress_single(
        multipart: Multipart,
    ) -> Result<Json<Vec<JobStatusResponse>>, (StatusCode, String)> {
        let uploads = collect_uploads(multipart).await?;
        execute_compression_job(uploads, process_uploads_sequential).await
    }

    // ======================================================================
    // UPLOAD HANDLING
    // ======================================================================

    async fn create_pending_upload(
        field: axum::extract::multipart::Field<'_>,
    ) -> Result<PendingUpload, (StatusCode, String)> {
        let name = field.file_name().map(|s| s.to_string());
        let data = field.bytes().await.map_err(internal_error)?;

        let original_filename = name.unwrap_or_else(|| "upload.pdf".to_string());
        let id = Uuid::new_v4().to_string();

        Ok(PendingUpload {
            stored_input_name: format!("{id}-{original_filename}"),
            original_filename,
            file_bytes: data.to_vec(),
        })
    }

    fn validate_uploads(
        uploads: Vec<PendingUpload>,
    ) -> Result<Vec<PendingUpload>, (StatusCode, String)> {
        match uploads.len() {
            0 => Err((StatusCode::BAD_REQUEST, "No file uploaded".into())),
            _ => Ok(uploads),
        }
    }



    async fn collect_uploads(
        mut multipart: Multipart,
    ) -> Result<Vec<PendingUpload>, (StatusCode, String)> {
        let mut uploads = Vec::new();

        while let Some(field) = multipart.next_field().await.map_err(internal_error)? {
            uploads.push(create_pending_upload(field).await?);
        }

        validate_uploads(uploads)
    }

    // ======================================================================
    // PARALLEL / SEQUENTIAL PROCESS
    // ======================================================================

    fn process_uploads_parallel(
        uploads: Vec<PendingUpload>,
    ) -> Result<Vec<JobStatusResponse>, String> {
        let upload_dir = UPLOAD_DIR.to_string();
        let compressed_dir = COMPRESSED_DIR.to_string();

        uploads
            .into_par_iter()
            .map(|u| process_single_upload(u, &upload_dir, &compressed_dir))
            .collect()
    }

    fn process_uploads_sequential(
        uploads: Vec<PendingUpload>,
    ) -> Result<Vec<JobStatusResponse>, String> {
        let upload_dir = UPLOAD_DIR.to_string();
        let compressed_dir = COMPRESSED_DIR.to_string();

        uploads
            .into_iter()
            .map(|u| process_single_upload(u, &upload_dir, &compressed_dir))
            .collect()
    }

    fn process_single_upload(
        upload: PendingUpload,
        upload_dir: &str,
        compressed_dir: &str,
    ) -> Result<JobStatusResponse, String> {
        let job_id = Uuid::new_v4().to_string();

        let PendingUpload { ref original_filename, .. } = upload;

        let (original_size, compressed_size, elapsed, output_path) =
            execute_file_operations(&upload, upload_dir, compressed_dir)?;

        let compressed_file_name =
            output_path.file_name().unwrap().to_string_lossy().to_string();

        Ok(build_job_status(
            job_id,
            original_filename.clone(),
            original_size,
            compressed_size,
            elapsed,
            compressed_file_name,
        ))
    }

    // ======================================================================
    // DOWNLOAD & ERROR
    // ======================================================================

    async fn handle_download(
        AxumPath(file): AxumPath<String>,
    ) -> Result<impl IntoResponse, (StatusCode, String)> {
        let path = PathBuf::from(COMPRESSED_DIR).join(&file);

        path.exists()
            .then_some(())
            .ok_or((StatusCode::NOT_FOUND, "File not found".into()))?;

        let bytes = fs::read(&path).map_err(internal_error)?;

        let file_name = path
            .file_name()
            .and_then(|f| f.to_str())
            .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "Invalid file name".into()))?;

        let dispo = HeaderValue::from_str(&format!("attachment; filename=\"{file_name}\""))
            .map_err(internal_error)?;

        Ok((
            [
                (header::CONTENT_TYPE, HeaderValue::from_static("application/pdf")),
                (header::CONTENT_DISPOSITION, dispo),
            ],
            bytes,
        ))
    }


    fn internal_error<E: std::fmt::Display>(err: E) -> (StatusCode, String) {
        (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
    }
