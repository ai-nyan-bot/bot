// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use axum::extract::State;
use axum::Json;
use log::debug;
use serde_json::{json, Value};

use crate::http::error::HttpError;
use crate::http::state::AppState;

pub async fn health(State(_state): State<AppState>) -> Result<Json<Value>, HttpError> {
    debug!("GET /health");

    let json_response = json!({
        "status": "ok"
    });

    Ok(Json(json_response))
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use serde_json::Value;

    use crate::http::testing::{extract, Test};

    #[tokio::test]
    async fn ok() {
        let test = Test::new().await;
        let response = test.get("/health").await;

        assert_eq!(response.status(), StatusCode::OK);

        let response: Value = extract(response).await.unwrap();
        assert_eq!(response.get("status").unwrap(), "ok");
    }
}
