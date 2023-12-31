use axum::{response::IntoResponse, http::StatusCode};


pub type Result<T> = core::result::Result<T,Error>;

#[derive(Debug)]
pub enum  Error {
    LoginFail,
    TicketDetailFailIdNotFound { id:u64 },
    NotAuthorizedError,
}



impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error{}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        println!("--> {:<12} - {self:?}", "INTO_RES");
        (StatusCode::INTERNAL_SERVER_ERROR, "CLIENT ERROR").into_response()
    }
}