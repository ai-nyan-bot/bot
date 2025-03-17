// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use std::ops::Deref;
use std::sync::Arc;

use rand::distributions::Alphanumeric;
use rand::Rng;
use sqlx::PgPool;

use crate::model::AuthToken;
use crate::repo::{AuthRepo, UserRepo, WalletRepo};
pub use authenticate::*;
use common::crypt::SecretKey;

mod authenticate;
mod get;
mod get_or_create;

pub type AuthTokenGenerator = fn() -> AuthToken;

#[derive(Clone)]
pub struct UserService(Arc<UserServiceInner>);

pub struct UserServiceInner {
    pool: PgPool,
    token_generator: AuthTokenGenerator,
    auth_repo: AuthRepo,
    user_repo: UserRepo,
    wallet_repo: WalletRepo,
}

impl Deref for UserService {
    type Target = UserServiceInner;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl UserService {
    pub fn new(pool: PgPool, secret: SecretKey) -> Self {
        Self(Arc::new(UserServiceInner {
            pool,
            token_generator: generate_random_token,
            auth_repo: AuthRepo::default(),
            user_repo: UserRepo::default(),
            wallet_repo: WalletRepo { secret },
        }))
    }

    pub fn new_with_token_generator(pool: PgPool, token_generator: AuthTokenGenerator) -> Self {
        Self(Arc::new(UserServiceInner {
            pool,
            token_generator,
            auth_repo: AuthRepo::default(),
            user_repo: UserRepo::default(),
            wallet_repo: WalletRepo::default(),
        }))
    }

    pub fn testing(pool: PgPool) -> Self {
        Self(Arc::new(UserServiceInner {
            pool,
            token_generator: generate_random_token,
            auth_repo: AuthRepo::default(),
            user_repo: UserRepo::default(),
            wallet_repo: WalletRepo {
                secret: SecretKey::from(
                    "276b49cc192cc66ab939de3892eba683152edab76c2162b21049d8fb0d9e7e5f",
                ),
            },
        }))
    }
}

fn generate_random_token() -> AuthToken {
    AuthToken(
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(128)
            .map(char::from)
            .collect(),
    )
}
