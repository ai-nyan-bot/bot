// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use std::fmt::Display;

use crate::repo::AuthCreateCmd;
use crate::service::user::UserService;
use crate::model::{Auth, User, Wallet};
use common::model::TelegramId;
use common::repo::error::RepoError;
use common::repo::Tx;
use common::service::{ServiceError, ServiceResult};

pub struct AuthenticateUserTelegramCmd {
    pub telegram_id: TelegramId,
}

impl UserService {
    pub async fn authenticate_and_create_telegram_user_if_not_exists(&self, cmd: AuthenticateUserTelegramCmd) -> ServiceResult<(User, Auth, Wallet)> {
        let mut tx = self.pool.begin().await?;

        let (user, wallet, _) = self.get_or_create_telegram_user_tx(&mut tx, cmd.telegram_id).await?;
        let auth = self.create_auth(&mut tx, &user).await?;

        tx.commit().await?;
        Ok((user, auth, wallet))
    }

    async fn create_auth(&self, tx: &mut Tx<'_>, user: &User) -> ServiceResult<Auth> {
        match self
            .auth_repo
            .create(
                tx,
                AuthCreateCmd {
                    user_id: user.id,
                    token: (self.token_generator)(),
                },
            )
            .await
        {
            Ok(auth) => Ok(auth),
            Err(err) => match err {
                RepoError::AlreadyExists => Err(ServiceError::conflict("Auth already exists")),
                _ => Err(ServiceError::internal(err.to_string())),
            },
        }
    }
}
