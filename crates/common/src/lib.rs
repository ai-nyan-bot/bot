// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

#![cfg_attr(not(debug_assertions), deny(warnings))]

pub use byte::*;
pub use config::*;
pub use signal::{Signal, SignalType};

mod byte;
mod config;
pub mod model;
pub mod repo;
pub mod service;

pub mod format;
mod signal;
