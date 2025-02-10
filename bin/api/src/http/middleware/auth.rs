// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use axum::extract::Request;
use axum::http;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use base::model::AuthenticatedUser;

pub async fn auth(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    let user = authenticate_user(req.headers()).await.ok_or(StatusCode::NOT_FOUND)?;
    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}

async fn authenticate_user(headers: &http::HeaderMap) -> Option<AuthenticatedUser> {
    if let Some(auth_header) = headers.get("Authorization") {
        let token = auth_header.to_str().ok()?;
        if token == "Bearer valid-token" {
            return Some(AuthenticatedUser { id: 1.into() });
        }
    }
    None
}
