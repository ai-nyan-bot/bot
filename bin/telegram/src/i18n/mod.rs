// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use serde::Deserialize;
use std::fmt::{Display, Formatter, Write};
use tokio::fs;

pub enum Language {
    De,
    En,
}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::De => f.write_str("de"),
            Language::En => f.write_str("en"),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct I18N {
    pub button_text_balance: String,
    pub button_text_receive: String,
    pub button_text_send: String,
    pub button_text_wallet: String,

    pub start: String,
    pub help: String,
}

impl I18N {
    pub async fn load(language: Language) -> Self {
        // FIXME wrap in once
        let crate_path = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");

        let content = fs::read_to_string(format!("{crate_path}/i18n/{language}.json"))
            .await
            .expect("Failed to read translations file");

        serde_json::from_str(&content).expect("Failed to parse translations")
    }
}
