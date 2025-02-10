// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::StrategyId;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct StrategyCreateRequest {}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StrategyCreateResponse {
    pub id: StrategyId,
}
