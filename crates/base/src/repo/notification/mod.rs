// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::UserId;
use common::model::Limit;
pub use create::*;

use std::ops::Deref;
use std::sync::Arc;

mod count;
mod create;
mod get;
mod pop;

pub struct NotificationQueryAll {
    pub limit: Limit,
}

pub struct NotificationQueryUser {
    pub user: UserId,
    pub limit: Limit,
}

#[derive(Debug, Clone)]
pub struct NotificationRepo(pub Arc<NotificationRepoInner>);

impl Deref for NotificationRepo {
    type Target = NotificationRepoInner;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[derive(Debug)]
pub struct NotificationRepoInner {}

impl NotificationRepo {
    pub fn new() -> Self {
        Self(Arc::new(NotificationRepoInner {}))
    }
}
