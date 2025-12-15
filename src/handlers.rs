use crate::{errors::ApiError, state::AppState, validated_query::ValidatedQuery};
use axum::{body::Body, extract::State, http::header, response::IntoResponse};
use serde::Deserialize;
use std::process::Stdio;
use tokio::process::Command;
use tokio_util::io::ReaderStream;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateFigureRequest {
    #[validate(url)]
    pub url: String,
}

#[axum::debug_handler]
pub async fn download_route(
    State(state): State<AppState>,
    ValidatedQuery(payload): ValidatedQuery<CreateFigureRequest>,
) -> impl IntoResponse {
    // passing url from user input is safe (at least on UNIX systems)

    // receiving filename
    let filename_output = match Command::new(&state.config.yt_dlp_path)
        .arg("-q")
        .arg("--print")
        .arg("filename")
        .arg("--restrict-filenames")
        .arg("-f")
        .arg("bestvideo[ext=mp4]+bestaudio[ext=m4a]/best[ext=mp4]/best")
        .arg("--max-filesize")
        .arg(&state.config.max_file_size)
        .arg("--")
        .arg(&payload.url)
        .output()
        .await
    {
        Ok(v) => v,
        Err(err) => {
            log::error!(
                "cannot call an yt-dlp to get a filename for {} cause of {err}",
                &payload.url
            );

            return ApiError::CannotDownloadInternal.into_response();
        }
    };

    let filename = String::from_utf8_lossy(&filename_output.stdout)
        .trim()
        .to_string();

    if filename.is_empty() {
        log::error!("filename is empty for {}", &payload.url);

        return ApiError::CannotDownloadBadRequest.into_response();
    }

    // downloading the media
    // we are trying to download the best quality MP4 variant. if there is no such, just the best one
    match Command::new(&state.config.yt_dlp_path)
        .arg("-q")
        .arg("-o")
        .arg("-")
        .arg("-f")
        .arg("bestvideo[ext=mp4]+bestaudio[ext=m4a]/best[ext=mp4]/best")
        .arg("--max-filesize")
        .arg(&state.config.max_file_size)
        .arg("--")
        .arg(&payload.url)
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(mut child) => {
            let stdout = child.stdout.take().unwrap();
            let stream = ReaderStream::new(stdout);

            let content_disposition = format!("attachment; filename=\"{}\"", filename);
            let mime_type = mime_guess::from_path(&filename)
                .first_or_octet_stream()
                .to_string();

            (
                [
                    (header::CONTENT_TYPE, mime_type),
                    (header::CONTENT_DISPOSITION, content_disposition),
                ],
                Body::from_stream(stream),
            )
                .into_response()
        }
        Err(err) => {
            log::error!(
                "cannot download a media for {} cause of {err}",
                &payload.url
            );

            ApiError::CannotDownloadInternal.into_response()
        }
    }
}
