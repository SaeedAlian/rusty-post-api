use actix_web::{http::StatusCode, HttpResponse as ActixHttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use std::fmt;

pub trait HttpResponse {
    fn new(message: impl Into<String>, status: u16) -> Self;

    fn into_http_response(self) -> ActixHttpResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultHttpResponse {
    pub message: String,
    pub status: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultHttpError {
    pub message: String,
    pub status: u16,
}

impl fmt::Display for DefaultHttpResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

impl fmt::Display for DefaultHttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "HttpError: message: {}, status: {}",
            self.message, self.status
        )
    }
}

impl std::error::Error for DefaultHttpError {}

impl ResponseError for DefaultHttpError {
    fn error_response(&self) -> ActixHttpResponse<actix_web::body::BoxBody> {
        let cloned = self.clone();
        cloned.into_http_response()
    }
}

impl HttpResponse for DefaultHttpResponse {
    fn new(message: impl Into<String>, status: u16) -> Self {
        DefaultHttpResponse {
            message: message.into(),
            status,
        }
    }

    fn into_http_response(self) -> ActixHttpResponse {
        let status = StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        ActixHttpResponse::build(status).json(self)
    }
}

impl HttpResponse for DefaultHttpError {
    fn new(message: impl Into<String>, status: u16) -> Self {
        DefaultHttpError {
            message: message.into(),
            status,
        }
    }

    fn into_http_response(self) -> ActixHttpResponse {
        let status = StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        ActixHttpResponse::build(status).json(self)
    }
}

impl DefaultHttpResponse {
    pub fn ok(message: impl Into<String>) -> Self {
        Self::new(message, 200)
    }
}

impl DefaultHttpError {
    pub fn server_error(message: impl Into<String>) -> Self {
        Self::new(message, 500)
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::new(message, 400)
    }

    pub fn unique_constraint_voilation(message: impl Into<String>) -> Self {
        Self::new(message, 409)
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::new(message, 401)
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(message, 404)
    }
}
