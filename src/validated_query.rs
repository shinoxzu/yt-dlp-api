use axum::{
    Json,
    extract::{FromRequestParts, Query, rejection::QueryRejection},
    http::{StatusCode, request::Parts},
    response::{IntoResponse, Response},
};
use serde::de::DeserializeOwned;
use thiserror::Error;
use validator::Validate;

use crate::dto::{ErrorDTO, ErrorWithValidationErrorsDTO};

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedQuery<T>(pub T);

impl<T, S> FromRequestParts<S> for ValidatedQuery<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Query<T>: FromRequestParts<S, Rejection = QueryRejection>,
{
    type Rejection = ServerError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Query(value) = Query::<T>::from_request_parts(parts, state).await?;
        value.validate()?;
        Ok(ValidatedQuery(value))
    }
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumFormRejection(#[from] QueryRejection),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::ValidationError(errors) => (
                StatusCode::BAD_REQUEST,
                Json(ErrorWithValidationErrorsDTO::new(
                    "validation errors ocurred".to_owned(),
                    errors,
                )),
            )
                .into_response(),
            ServerError::AxumFormRejection(r) => {
                (StatusCode::BAD_REQUEST, Json(ErrorDTO::new(r.body_text()))).into_response()
            }
        }
        .into_response()
    }
}
