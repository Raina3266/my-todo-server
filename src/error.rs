use axum::{Json, http::StatusCode, response::{ErrorResponse, IntoResponse, Response}};

pub type Result<T, E = Error> = ::core::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
    TodoNotFound,
    Unexpected,
}

impl Error {
    pub fn code_str(&self) -> &'static str {
        match self {
            Self::TodoNotFound => "To do not found",
            Self::Unexpected => "unexpected",
        }
    }
}

// impl IntoResponse for () {}
// impl<T: IntoResponse, E: IntoResponse> IntoResponse for Result<T, E> {}
// impl<T: Serialize> IntoResponse for Json<T> {}
// impl<T: IntoResponse> IntoResponse for (StatusCode, T) {}
// impl IntoResponse for StatusCode {} => impl IntoResponse for (StatusCode, ()) {}
// 
// impl IntoResponse for (StatusCode, StatusCode) {}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status = match self {
            Self::TodoNotFound => StatusCode::NOT_FOUND,
            Self::Unexpected => StatusCode::INTERNAL_SERVER_ERROR,
        };
        
        let body = serde_json::json!({
            "code": self.code_str(),
        });
        
        (status, Json(body)).into_response()
    }
}