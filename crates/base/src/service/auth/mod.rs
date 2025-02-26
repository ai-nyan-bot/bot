// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use std::ops::Deref;
use std::sync::Arc;

use crate::repo::AuthRepo;
use sqlx::PgPool;

mod get;

#[derive(Clone)]
pub struct AuthService(Arc<AuthServiceInner>);

pub struct AuthServiceInner {
    pool: PgPool,
    auth_repo: AuthRepo,
}

impl Deref for AuthService {
    type Target = AuthServiceInner;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl AuthService {
    pub fn new(pool: PgPool, auth_repo: AuthRepo) -> Self {
        Self(Arc::new(AuthServiceInner { pool, auth_repo }))
    }

    pub fn testing(pool: PgPool) -> Self {
        Self(Arc::new(AuthServiceInner {
            pool,
            auth_repo: AuthRepo::new(),
        }))
    }
}
