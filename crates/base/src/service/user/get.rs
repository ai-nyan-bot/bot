// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{User, UserId};
use crate::service::UserService;
use common::repo::error::RepoError;
use common::service::{ServiceError, ServiceResult};

impl UserService {
    pub async fn get_by_id(&self, id: impl Into<UserId> + Send) -> ServiceResult<User> {
        let mut tx = self.pool.begin().await?;

        let result = match self.user_repo.get_by_id(&mut tx, id).await {
            Ok(user) => user,
            Err(err) => {
                return match err {
                    RepoError::NotFound => Err(ServiceError::not_found("User not found")),
                    _ => Err(err.into()),
                }
            }
        };
        
        tx.commit().await?;
        Ok(result)
    }
}
