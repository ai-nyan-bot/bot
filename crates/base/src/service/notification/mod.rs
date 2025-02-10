// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

pub use create::NotificationConditionMet;

mod create;
mod delete;

use crate::repo::NotificationRepo;
use sqlx::PgPool;
use std::ops::Deref;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct NotificationService(pub Arc<NotificationServiceInner>);

impl Deref for NotificationService {
    type Target = NotificationServiceInner;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[derive(Debug)]
pub struct NotificationServiceInner {
    pool: PgPool,
    repo: NotificationRepo,
}

impl NotificationService {
    pub fn new(pool: PgPool, repo: NotificationRepo) -> Self {
        Self(Arc::new(NotificationServiceInner { pool, repo }))
    }
}
