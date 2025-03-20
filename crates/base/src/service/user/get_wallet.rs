// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{UserId, Wallet};
use crate::service::UserService;
use common::repo::Tx;
use common::service::{ServiceError, ServiceResult};

impl UserService {

    pub async fn get_wallet(&self, user: impl Into<UserId> + Send) -> ServiceResult<Wallet> {
        let mut tx = self.pool.begin().await?;
        let result = self.get_wallet_tx(&mut tx, user.into()).await?;
        tx.commit().await?;
        Ok(result)
    }

    pub async fn get_wallet_tx(
        &self,
        tx: &mut Tx<'_>,
        user: impl Into<UserId> + Send,
    ) -> ServiceResult<Wallet> {
        match self.wallet_repo.get_by_user_id(tx, user).await {
            Ok(wallet) => Ok(wallet),
            Err(_) => Err(ServiceError::not_found("Wallet not found".to_string())),
        }
    }
}
