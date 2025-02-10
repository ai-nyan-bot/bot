// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::Sequence;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Action {
    AndThen(Box<Action>, Box<Sequence>),
    Buy,
    Notify,
    Sell,
}
