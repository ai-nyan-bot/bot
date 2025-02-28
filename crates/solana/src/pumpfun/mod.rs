// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/nhuxhr/pumpfun-rs (MIT License).
// Original MIT License Copyright (c) nhuxhr 2024.

pub(crate) mod constant;
pub mod model;
pub mod parse;
pub mod repo;
pub mod rpc;
pub mod service;
mod tx;
pub(crate) mod util;

pub use crate::pumpfun::parse::PumpFunParser;
pub use rpc::Rpc;
use std::fmt::{Debug, Display, Formatter};

pub struct Pumpfun {}

#[derive(Debug)]
pub enum PumpfunError {
    CurveCompleted,
}

impl Display for PumpfunError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PumpfunError::CurveCompleted => f.write_str("Curve already completed"),
        }
    }
}

impl std::error::Error for PumpfunError {}

pub type PumpfunResult<T> = std::result::Result<T, PumpfunError>;
