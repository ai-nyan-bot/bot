// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::config::Config;
use base::repo::NotificationRepo;
use base::service::{NotificationService, UserService};
use common::repo::pool::setup_pool;
use std::ops::Deref;
use std::sync::Arc;
use testing::get_test_pool;

#[derive(Clone)]
pub struct AppState(pub Arc<AppStateInner>);

impl AppState {
    pub fn user_service(&self) -> UserService {
        self.service.user.clone()
    }
}

impl Deref for AppState {
    type Target = AppStateInner;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

pub struct AppStateInner {
    pub config: Config,
    pub service: Service,
}

pub struct Service {
    pub user: UserService,
    pub notification: NotificationService,
}

impl AppState {
    pub async fn setup(config: Config) -> Self {
        let pool = setup_pool(&config.postgres).await;

        Self(Arc::new(AppStateInner {
            config,
            service: Service {
                notification: NotificationService::new(pool.clone(), NotificationRepo::new()),
                user: UserService::new(pool),
            },
        }))
    }

    pub async fn testing(config: Config) -> Self {
        let pool = get_test_pool().await;

        Self(Arc::new(AppStateInner {
            config,
            service: Service {
                notification: NotificationService::new(pool.clone(), NotificationRepo::new()),
                user: UserService::new(pool.clone()),
            },
        }))
    }
}
