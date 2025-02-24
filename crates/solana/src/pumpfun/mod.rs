// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/nhuxhr/pumpfun-rs (MIT License).
// Original MIT License Copyright (c) nhuxhr 2024.

pub mod model;
pub mod repo;
pub mod rpc;

pub(crate) mod constant;
mod tx;
pub mod parse;
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
