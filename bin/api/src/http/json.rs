// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use axum::extract::rejection::JsonRejection;
use axum_macros::FromRequest;

use crate::http::error::HttpError;

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(HttpError))]
pub struct JsonReq<T>(pub T);

impl From<JsonRejection> for HttpError {
    fn from(rejection: JsonRejection) -> Self {
        match rejection {
            JsonRejection::JsonDataError(err) => HttpError::unprocessable(err.to_string()),
            JsonRejection::JsonSyntaxError(err) => HttpError::bad_request(err.to_string()),
            JsonRejection::MissingJsonContentType(_) => {
                HttpError::bad_request("Request needs to be of content type application/json")
            }
            _ => HttpError::internal_server("Internal server error"),
        }
    }
}
