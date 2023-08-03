use actix_web::{HttpResponseBuilder, ResponseError};
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct AppError {
    pub message: Option<String>,
    pub cause: Option<Box<dyn std::error::Error>>,
    pub status: u16,
}

#[derive(Debug, Serialize)]
struct AppErrorBody {
    pub message: String,
}

impl AppError {
    pub fn new(status: u16) -> Self {
        AppError {
            status,
            message: None,
            cause: None,
        }
    }

    pub fn message(mut self, message: &str) -> Self {
        self.message = Some(message.to_string());
        self
    }

    pub fn cause<E: std::error::Error + 'static>(mut self, cause: E) -> Self {
        self.cause = Some(Box::new(cause));
        self
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match (&self.cause, &self.message) {
            (Some(cause), Some(message)) => write!(f, "{}: {}", message, cause),
            (Some(cause), None) => write!(f, "{}", cause),
            (None, Some(message)) => write!(f, "{}", message),
            (None, None) => write!(f, "{}", self.status_code().canonical_reason().unwrap()),
        }
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::from_u16(self.status).unwrap()
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        HttpResponseBuilder::new(self.status_code()).json(AppErrorBody {
            message: format!("{}", self),
        })
    }
}
