// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::{Field, Operator, RuleId, RuleName, Value};
use common::model::Timeframe;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct RuleCreateRequest {}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleCreateResponse {
    pub id: RuleId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RuleUpdateRequest {
    pub name: Option<RuleName>,
    pub sequence: Option<ApiSequence>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleUpdateResponse {
    pub id: RuleId,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpRuleList {
    pub id: RuleId,
    pub name: RuleName,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpRuleListResponse {
    pub rules: Box<[HttpRuleList]>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpRulGetResponse {
    pub id: RuleId,
    pub name: RuleName,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ApiCondition {
    And {
        conditions: Vec<ApiCondition>,
    },
    Compare {
        field: Field,
        operator: Operator,
        value: Value,
        timeframe: Option<Timeframe>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiSequence {
    pub condition: ApiCondition,
}
