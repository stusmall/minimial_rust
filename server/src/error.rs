use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::error::Error as StdError;
use std::fmt::{Debug, Display, Formatter};

use deadpool_postgres::tokio_postgres::Error as PostgresError;
use deadpool_postgres::PoolError;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug)]
pub enum Error {
    PoolError(PoolError),
    PostgresError(PostgresError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::PoolError(e) => Display::fmt(&e, f),
            Error::PostgresError(e) => Display::fmt(&e, f),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::PoolError(e) => Some(e),
            Error::PostgresError(e) => Some(e),
        }
    }
}

impl From<PoolError> for Error {
    fn from(e: PoolError) -> Self {
        Error::PoolError(e)
    }
}

impl From<PostgresError> for Error {
    fn from(e: PostgresError) -> Self {
        Error::PostgresError(e)
    }
}
#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    cause: String,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::PostgresError(_) | Error::PoolError(_) => {
                let mut resp = axum::Json(ErrorResponse {
                    cause: String::from("Failure to run database query"),
                })
                .into_response();
                (*resp.status_mut()) = StatusCode::INTERNAL_SERVER_ERROR;
                resp
            }
        }
    }
}
