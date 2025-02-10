// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use std::borrow::Cow;
use std::collections::HashMap;

use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use base::model::AuthenticatedUser;

pub async fn auth(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    if let Some(query) = req.uri().query() {
        let query_params: HashMap<_, _> = url::form_urlencoded::parse(query.as_bytes()).collect();
        let user = authenticate_user(query_params).await.ok_or(StatusCode::NOT_FOUND)?;
        req.extensions_mut().insert(user);
        return Ok(next.run(req).await);
    }

    todo!()
}

async fn authenticate_user(query: HashMap<Cow<'_, str>, Cow<'_, str>>) -> Option<AuthenticatedUser> {
    if let Some(access_token) = query.get("access_token") {
        if access_token == "valid-token" {
            return Some(AuthenticatedUser { id: 1.into() });
        }
    }
    None
}
