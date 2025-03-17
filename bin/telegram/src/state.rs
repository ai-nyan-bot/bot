// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::callback::CallbackStore;
use crate::config::Config;
use crate::{TelegramConfig, WalletConfig};
use base::repo::{NotificationRepo, RuleRepo, TokenPairRepo, TokenRepo};
use base::service::{NotificationService, RuleService, TokenService, UserService};
use base::test::NeverCalledTokenInfoLoader;
use common::crypt::SecretKey;
use common::repo::pool::{setup_pool, PostgresConfig};
use common::ConfigValue;
use solana::pumpfun;
use solana::pumpfun::repo::{CurveRepo, SummaryRepo};
use sqlx::{Pool, Postgres};
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;
use teloxide::Bot;

#[derive(Clone)]
pub struct AppState(pub Arc<AppStateInner>);

impl AppState {
    pub fn notification_service(&self) -> NotificationService {
        self.service.notification.clone()
    }

    pub fn token_service(&self) -> TokenService {
        self.service.token.clone()
    }

    pub fn pumpfun_token_service(&self) -> pumpfun::service::TokenService {
        self.service.pumpfun_token_service.clone()
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
    pub callback_store: CallbackStore,
    pub service: Service,
}

pub struct Service {
    pub notification: NotificationService,
    pub pumpfun_token_service: pumpfun::service::TokenService,
    pub rule: RuleService,
    pub token: TokenService,
    pub user: UserService,
}

impl AppState {
    pub async fn setup(config: Config) -> Self {
        let pool = setup_pool(&config.postgres).await;
        let bot = Bot::new(config.telegram.token.resolve());

        let token_repo = TokenRepo::new(Box::new(NeverCalledTokenInfoLoader {}));
        let token_pair_repo = TokenPairRepo::new(token_repo.clone());
        let secret = SecretKey::from(config.wallet.secret.resolve());

        Self(Arc::new(AppStateInner {
            config,
            bot,
            callback_store: CallbackStore::new(Duration::from_secs(60 * 15)),
            service: Service {
                notification: NotificationService::new(pool.clone(), NotificationRepo::new()),
                pumpfun_token_service: pumpfun::service::TokenService::new(
                    pool.clone(),
                    token_pair_repo.clone(),
                    CurveRepo::new(),
                    SummaryRepo::new(),
                ),
                rule: RuleService::new(pool.clone(), RuleRepo::new()),
                token: TokenService::new(pool.clone(), token_pair_repo.clone()),
                user: UserService::new(pool, secret),
            },
        }))
    }

    pub async fn testing(pool: Pool<Postgres>) -> Self {
        let bot = Bot::new("1234567890:QWERTYUIOPASDFGHJKLZXCVBNMQWERTYUIO");

        Self(Arc::new(AppStateInner {
            config: Config {
                telegram: TelegramConfig {
                    token: ConfigValue::Value(
                        "1234567890:QWERTYUIOPASDFGHJKLZXCVBNMQWERTYUIO".to_string(),
                    ), // same as mockbot
                    webapp_url: ConfigValue::Value("https://test.nyanbot.com".to_string()),
                },
                postgres: PostgresConfig::default(),
                wallet: WalletConfig {
                    secret: ConfigValue::Value(
                        "c004a55d744672f98c9e996fe4b8c1b33cea79e9afeafca918a6a36e09777b7e"
                            .to_string(),
                    ),
                },
            },
            bot,
            callback_store: CallbackStore::new(Duration::from_secs(1)),
            service: Service {
                notification: NotificationService::testing(pool.clone()),
                pumpfun_token_service: pumpfun::service::TokenService::testing(pool.clone()),
                rule: RuleService::testing(pool.clone()),
                token: TokenService::testing(pool.clone()),
                user: UserService::new(
                    pool.clone(),
                    SecretKey::from(
                        "c004a55d744672f98c9e996fe4b8c1b33cea79e9afeafca918a6a36e09777b7e",
                    ),
                ),
            },
        }))
    }
}
