use axum::{
    Json,
    extract::rejection::QueryRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Serialize, Clone)]
pub struct ErrorDTO {
    pub message: String,
}

impl ErrorDTO {
    pub fn new(message: impl Into<String>) -> Self {
        ErrorDTO {
            message: message.into(),
        }
    }
}

#[derive(Serialize, Clone)]
pub struct ErrorWithValidationErrorsDTO {
    pub message: String,
    pub errors: ValidationErrors,
}

impl ErrorWithValidationErrorsDTO {
    pub fn new(message: String, errors: ValidationErrors) -> Self {
        ErrorWithValidationErrorsDTO { message, errors }
    }
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumFormRejection(#[from] QueryRejection),

    #[error("cannot download (internal error)")]
    CannotDownloadInternal,

    #[error("cannot download (bad request)")]
    CannotDownloadBadRequest,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::ValidationError(errors) => (
                StatusCode::BAD_REQUEST,
                Json(ErrorWithValidationErrorsDTO::new(
                    "validation errors ocurred".to_owned(),
                    errors,
                )),
            )
                .into_response(),
            ApiError::AxumFormRejection(r) => {
                (StatusCode::BAD_REQUEST, Json(ErrorDTO::new(r.body_text()))).into_response()
            }
            ApiError::CannotDownloadBadRequest => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ErrorDTO::new(
                        "sorry, cannot download this (the reason is likely a bad URL).",
                    )),
                )
                    .into_response();
            }
            ApiError::CannotDownloadInternal => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorDTO::new(
                        "sorry, cannot download this (the reason likely on our side).",
                    )),
                )
                    .into_response();
            }
        }
        .into_response()
    }
}
