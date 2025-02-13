// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::{RuleId, RuleName};
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
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ApiConditionType {
    And,
    Compare,
    Or,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ApiField {
    Price,
    Trades,
    Volume,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ApiOperator {
    GreaterThan,
    IncreasedBy,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ApiTimeframe {
    M1,
    M5,
    M15,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ApiValue {
    Boolean { value: bool },
    Money { value: f64 },
    Percent { value: f64 },
    String { value: String },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ApiCondition {
    And {
        conditions: Option<Vec<ApiCondition>>,
    },
    Compare {
        field: ApiField,
        operator: ApiOperator,
        value: ApiValue,
        timeframe: Option<ApiTimeframe>,
    },
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct ApiCondition {
//     pub r#type: ApiConditionType,
//     pub field: Option<ApiField>,
//     pub operator: Option<ApiOperator>,
//     pub value: Option<ApiValue>,
//     pub timeframe: Option<ApiTimeframe>,
//     pub conditions: Option<Vec<ApiCondition>>,
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiSequence {
    pub condition: ApiCondition,
}
