// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{User, UserId};
use crate::service::UserService;
use common::service::ServiceResult;

impl UserService {
    pub async fn get_by_id(&self, id: impl Into<UserId> + Send) -> ServiceResult<User> {
        let mut tx = self.pool.begin().await?;
        let result = self.user_repo.get_by_id(&mut tx, id).await?;
        tx.commit().await?;
        Ok(result)
    }
}
