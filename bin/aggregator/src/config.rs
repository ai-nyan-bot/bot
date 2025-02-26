// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use std::env::args;
use std::path::PathBuf;
use std::str::FromStr;

use log::info;
use serde::Deserialize;

use common::repo::pool::PostgresConfig;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Config {
    pub postgres: PostgresConfig,
}

impl Config {
    pub fn load() -> Self {
        let args: Vec<String> = args().collect();

        let config_path = if args.len() == 2 {
            PathBuf::from_str(args.get(1).unwrap()).unwrap()
        } else {
            let path = PathBuf::from_str(args.first().unwrap()).unwrap();
            path.parent().unwrap().join("config.toml")
        };

        info!("Loads {}", config_path.to_string_lossy());
        let config = std::fs::read_to_string(config_path).expect("Unable to read config");

        toml::from_str(&config).expect("Unable to parse config")
    }
}
