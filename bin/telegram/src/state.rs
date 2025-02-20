// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::config::Config;
use base::repo::{NotificationRepo, ReadTokenPairRepo, ReadTokenRepo, RuleRepo};
use base::service::{NotificationService, RuleService, TokenService, UserService};
use common::repo::pool::setup_pool;
use std::ops::Deref;
use std::sync::Arc;
use teloxide::Bot;
use testing::get_test_pool;

#[derive(Clone)]
pub struct AppState(pub Arc<AppStateInner>);

impl AppState {
    pub fn notification_service(&self) -> NotificationService {
        self.service.notification.clone()
    }

    pub fn token_service(&self) -> TokenService {
        self.service.token.clone()
    }

    pub fn rule_service(&self) -> RuleService {
        self.service.rule.clone()
    }

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
    pub bot: Bot,
    pub service: Service,
}

pub struct Service {
    pub notification: NotificationService,
    pub rule: RuleService,
    pub token: TokenService,
    pub user: UserService,
}

impl AppState {
    pub async fn setup(config: Config) -> Self {
        let pool = setup_pool(&config.postgres).await;
        let bot = Bot::new(config.telegram.token.resolve());

        let token_repo = ReadTokenRepo::new();
        let token_pair_repo = ReadTokenPairRepo::new(token_repo.clone());

        Self(Arc::new(AppStateInner {
            config,
            bot,
            service: Service {
                notification: NotificationService::new(pool.clone(), NotificationRepo::new()),
                rule: RuleService::new(pool.clone(), RuleRepo::new()),
                token: TokenService::new(pool.clone(), token_repo.clone(), token_pair_repo.clone()),
                user: UserService::new(pool),
            },
        }))
    }

    pub async fn testing(config: Config) -> Self {
        let pool = get_test_pool().await;
        let bot = Bot::new(config.telegram.token.resolve());

        Self(Arc::new(AppStateInner {
            config,
            bot,
            service: Service {
                notification: NotificationService::testing(pool.clone()),
                rule: RuleService::testing(pool.clone()),
                token: TokenService::testing(pool.clone()),
                user: UserService::new(pool.clone()),
            },
        }))
    }
}
