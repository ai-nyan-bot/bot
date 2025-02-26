// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{AuthToken, AuthenticatedUser};
use crate::service::AuthService;
use common::repo::error::RepoError;
use common::service::{ServiceError, ServiceResult};

impl AuthService {
    pub async fn get_by_token(&self, token: impl Into<AuthToken> + Send) -> ServiceResult<AuthenticatedUser> {
        let mut tx = self.pool.begin().await?;
        let result = match self.auth_repo.get_by_token(&mut tx, token).await {
            Ok(auth) => auth,
            Err(err) => return match err {
                RepoError::NotFound => Err(ServiceError::not_found("User not found")),
                _ => Err(err.into()),
            },
        };
        tx.commit().await?;
        Ok(result.user)
    }
}
