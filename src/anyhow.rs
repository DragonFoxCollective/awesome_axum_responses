use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use derive_more::Display;

pub use anyhow::anyhow;

pub type Result<T, Err = Error> = std::result::Result<T, Err>;

#[derive(Debug, Display)]
pub struct Error(anyhow::Error);

impl<T> From<T> for Error
where
    T: Into<anyhow::Error>,
{
    fn from(value: T) -> Self {
        Error(value.into())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        tracing::error!(err = ?self.0, "responding with error");
        (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string()).into_response()
    }
}
