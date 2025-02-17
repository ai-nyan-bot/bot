// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use common::service::ServiceError;
use serde::{Deserialize, Deserializer};
use serde_json::json;

#[derive(Debug, Deserialize, PartialEq)]
pub enum HttpError {
    BadRequest(String),
    Conflict(String),
    InternalServer(String),
    NotFound(String),
    Unprocessable(String),
}

impl HttpError {
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::BadRequest(message.into())
    }

    pub fn conflict(message: impl Into<String>) -> Self {
        Self::Conflict(message.into())
    }

    pub fn internal_server(message: impl Into<String>) -> Self {
        Self::InternalServer(message.into())
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self::NotFound(message.into())
    }

    pub fn unprocessable(message: impl Into<String>) -> Self {
        Self::Unprocessable(message.into())
    }
}

impl From<ServiceError> for HttpError {
    fn from(value: ServiceError) -> Self {
        match value {
            ServiceError::Conflict(message) => HttpError::Conflict(message),
            ServiceError::Internal(_) => HttpError::InternalServer("Internal server error".to_string()),
            ServiceError::NotFound(message) => HttpError::NotFound(message),
        }
    }
}

pub struct HttpErrorResponse {
    pub code: StatusCode,
    pub message: String,
}

impl<'de> Deserialize<'de> for HttpErrorResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct ErrorResponse {
            error: Error,
        }
        #[derive(Deserialize)]
        struct Error {
            code: u16,
            message: String,
        }

        let response: ErrorResponse = Deserialize::deserialize(deserializer)?;
        Ok(HttpErrorResponse {
            code: StatusCode::from_u16(response.error.code).unwrap(),
            message: response.error.message,
        })
    }
}

impl From<HttpError> for HttpErrorResponse {
    fn from(value: HttpError) -> Self {
        match value {
            HttpError::BadRequest(message) => HttpErrorResponse {
                code: StatusCode::BAD_REQUEST,
                message,
            },
            HttpError::Conflict(message) => HttpErrorResponse {
                code: StatusCode::CONFLICT,
                message,
            },
            HttpError::InternalServer(message) => HttpErrorResponse {
                code: StatusCode::INTERNAL_SERVER_ERROR,
                message,
            },
            HttpError::NotFound(message) => HttpErrorResponse {
                code: StatusCode::NOT_FOUND,
                message,
            },
            HttpError::Unprocessable(message) => HttpErrorResponse {
                code: StatusCode::UNPROCESSABLE_ENTITY,
                message,
            },
        }
    }
}

impl IntoResponse for HttpErrorResponse {
    fn into_response(self) -> Response {
        let status = self.code;
        let body = Json(json!({
            "error": {
                "code": status.as_u16(),
                "message": self.message
            }
        }));
        (status, body).into_response()
    }
}

// Tell axum how to convert `HttpError` into a response.
impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        let error_response: HttpErrorResponse = self.into();
        error_response.into_response()
    }
}
