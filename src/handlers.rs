use crate::{dto::ErrorDTO, state::AppState, validated_query::ValidatedQuery};
use axum::{
    Json,
    body::Body,
    extract::State,
    http::{StatusCode, header},
    response::IntoResponse,
};
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
    let filename_output = match Command::new("yt-dlp")
        .arg("--print")
        .arg("filename")
        .arg("--restrict-filenames")
        .arg("-f")
        .arg("bestvideo[ext=mp4]+bestaudio[ext=m4a]/best[ext=mp4]/best")
        .arg("--max-filesize")
        .arg(&state.config.max_file_size)
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

            return (
                StatusCode::BAD_REQUEST,
                Json(ErrorDTO::new("sorry, cannot download this")),
            )
                .into_response();
        }
    };

    let filename = match String::from_utf8(filename_output.stdout) {
        Ok(v) => {
            let filename = v.trim().to_string();

            if filename.is_empty() {
                log::error!("filename is empty for {}", &payload.url);

                return (
                    StatusCode::BAD_REQUEST,
                    Json(ErrorDTO::new("sorry, cannot download this")),
                )
                    .into_response();
            }

            filename
        }
        Err(err) => {
            log::error!(
                "cannot fetch a filename from yt-dlp for {} cause of {err}",
                &payload.url
            );

            return (
                StatusCode::BAD_REQUEST,
                Json(ErrorDTO::new("sorry, cannot download this")),
            )
                .into_response();
        }
    };

    // downloading the media
    match Command::new("yt-dlp")
        .arg("-o")
        .arg("-")
        .arg("-f")
        .arg("bestvideo[ext=mp4]+bestaudio[ext=m4a]/best[ext=mp4]/best")
        .arg("--max-filesize")
        .arg(&state.config.max_file_size)
        .arg(&payload.url)
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(mut child) => {
            let stdout = child.stdout.take().unwrap();
            let stream = ReaderStream::new(stdout);

            let content_disposition = format!("attachment; filename=\"{}\"", filename);

            (
                [
                    (header::CONTENT_TYPE, "video/mp4".to_string()),
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

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorDTO::new("sorry, cannot download this")),
            )
                .into_response()
        }
    }
}
