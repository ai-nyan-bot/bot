// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::{RuleId, RuleName, Sequence};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct HttpRuleCreateRequest {
    pub name: RuleName,
    pub sequence: Sequence,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpRuleCreateResponse {
    pub id: RuleId,
    pub name: RuleName,
    pub sequence: Sequence,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HttpRuleUpdateRequest {
    pub name: Option<RuleName>,
    pub sequence: Option<Sequence>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpRuleUpdateResponse {
    pub id: RuleId,
    pub name: RuleName,
    pub sequence: Sequence,
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
    pub sequence: Sequence,
}
