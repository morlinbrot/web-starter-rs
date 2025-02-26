use axum::{
    extract::{rejection::JsonRejection, FromRequest},
    response::{IntoResponse, Response},
};
use serde::Serialize;
use tracing::error;

// See axum example https://github.com/tokio-rs/axum/blob/main/examples/error-handling/src/main.rs

pub type Result<T> = core::result::Result<T, Error>;

// The kinds of errors we can hit in our application.
pub enum Error {
    // The request body contained invalid JSON
    JsonRejection(JsonRejection),
}

// Create our own JSON extractor by wrapping `axum::Json`. This makes it easy to override the
// rejection and provide our own which formats errors to match our application.
//
// `axum::Json` responds with plain text if the input is invalid.
#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(Error))]
pub struct AppJson<T>(pub T);

impl<T> IntoResponse for AppJson<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}

// Tell axum how `HandlerError` should be converted into a response.
//
// This is also a convenient place to log errors.
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        // How we want errors responses to be serialized
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        let (status, message) = match self {
            Error::JsonRejection(rejection) => {
                error!("Failed to parse JSON");
                (rejection.status(), rejection.body_text())
            }
        };

        (status, AppJson(ErrorResponse { message })).into_response()
    }
}

impl From<JsonRejection> for Error {
    fn from(rejection: JsonRejection) -> Self {
        Self::JsonRejection(rejection)
    }
}
