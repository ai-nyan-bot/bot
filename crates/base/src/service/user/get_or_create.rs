// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::KeyPair;
use crate::model::{User, Wallet};
use crate::repo::{UserCreateTelegramCmd, WalletCreateCmd};
use crate::service::user::UserService;
use common::crypt::Nonce;
use common::model::TelegramId;
use common::repo::error::RepoError;
use common::repo::Tx;
use common::service::{ServiceError, ServiceResult};

impl UserService {
    pub async fn get_or_create_telegram_user(
        &self,
        telegram_id: impl Into<TelegramId>,
    ) -> ServiceResult<(User, Wallet, bool)> {
        let mut tx = self.pool.begin().await?;
        let result = self
            .get_or_create_telegram_user_tx(&mut tx, telegram_id)
            .await?;
        tx.commit().await?;
        Ok(result)
    }

    pub async fn get_or_create_telegram_user_tx(
        &self,
        tx: &mut Tx<'_>,
        telegram_id: impl Into<TelegramId>,
    ) -> ServiceResult<(User, Wallet, bool)> {
        let telegram_id = telegram_id.into();
        match self
            .user_repo
            .get_by_telegram_id(tx, telegram_id.clone())
            .await
        {
            Ok(user) => {
                let wallet = self.get_wallet_tx(tx, &user).await?;
                Ok((user, wallet, false))
            }
            Err(_) => match self
                .user_repo
                .create_telegram(tx, UserCreateTelegramCmd { telegram_id })
                .await
            {
                Ok(user) => {
                    let wallet = self.create_wallet(tx, &user).await?;
                    Ok((user, wallet, true))
                }
                Err(err) => match err {
                    RepoError::AlreadyExists => Err(ServiceError::conflict("User already exists")),
                    _ => Err(ServiceError::internal(err.to_string())),
                },
            },
        }
    }

    async fn create_wallet(&self, tx: &mut Tx<'_>, user: &User) -> ServiceResult<Wallet> {
        let keypair = KeyPair::generate();
        match self
            .wallet_repo
            .create(
                tx,
                WalletCreateCmd {
                    user_id: user.id,
                    public_key: keypair.public,
                    private_key: keypair.private,
                    nonce: Nonce::generate(),
                },
            )
            .await
        {
            Ok(wallet) => Ok(wallet),
            Err(err) => match err {
                RepoError::AlreadyExists => Err(ServiceError::conflict("Wallet already exists")),
                _ => Err(ServiceError::internal(err.to_string())),
            },
        }
    }
}
