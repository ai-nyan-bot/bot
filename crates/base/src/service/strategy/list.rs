// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::Strategy;
use crate::repo::StrategyQueryAll;
use crate::service::strategy::StrategyService;
use common::model::Limit;
use common::service::ServiceResult;

impl StrategyService {
    pub async fn list_active(&self) -> ServiceResult<Box<[Strategy]>> {
        let mut tx = self.pool.begin().await?;
        let result = self.repo.list_all(&mut tx, StrategyQueryAll { limit: Limit::unlimited() }).await?;
        tx.commit().await?;
        Ok(result)
    }
}
