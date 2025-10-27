use axum::{
    extract::Path,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};
use mime_guess;
use std::{fs::File, io::Read, path::PathBuf};
use log::{
    info, error
};
#[cfg(debug_assertions)]
use log::debug;

/// # File Download API written in RUST.
/// 
/// ## Parameters
/// - path : path to file in relative path.
#[utoipa::path(
    get,
    path = "/download/{path}",
    params(
        ("path" = String, Path, description = "Path to target file")
    ),
    responses(
        (status = 200, description = "File download", content_type = "application/octet-stream")
    )
)]
pub async fn download(Path(path): Path<PathBuf>) -> Response {
    let mut full_path = PathBuf::from("./drive");
    full_path.push(PathBuf::from(&path));

    let file_name = full_path
        .file_name() 
        .unwrap()
        .to_string_lossy();

    if !full_path.exists() {
        error!("File not found error occur : {}", path.to_str().unwrap());
        return (StatusCode::NOT_FOUND, "File not found").into_response();
    }

    #[cfg(debug_assertions)]
    match full_path.to_str() {
        Some(v) => debug!("Check file name {}", v),
        None => debug!("Wrong file name.")
    }

    match File::open(&full_path) {
        Ok(mut file) => {
            let mut buf = Vec::new();
            if let Err(_) = file.read_to_end(&mut buf) {
                error!("Read error occur!");
                return (StatusCode::INTERNAL_SERVER_ERROR, "Read error").into_response();
            }
            let mime = mime_guess::from_path(&full_path).first_or_octet_stream();
            info!("File send : {}", path.to_str().unwrap());
            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, mime.as_ref())
                .header(
                    header::CONTENT_DISPOSITION,
                    format!("attachment; filename=\"{}\"", file_name),
                )
                .body(buf.into())
                .unwrap()
        }
        Err(_) => {
            error!("Some critical file error occur : {}", path.to_str().unwrap());
            (StatusCode::NOT_FOUND, "File not found").into_response()
        }
    }
}