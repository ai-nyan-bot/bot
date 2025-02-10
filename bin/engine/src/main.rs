// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::config::Config;
use crate::fact::FactService;
use crate::state::{AppState, AppStateInner, Service};
use base::model::Action;
use base::repo::{InvocationCreateCmd, InvocationRepo, NotificationRepo, StrategyRepo};
use base::service::{NotificationConditionMet, NotificationService, StrategyService};
use common::repo::pool::setup_pool;
use solana::repo::pumpfun::ReadTradeRepo;
use std::sync::Arc;
use std::time::Duration;
use tokio::runtime::Builder;
use tokio::time::sleep;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

mod config;
mod fact;
mod state;

fn main() {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::load();
    println!("Starting the engine");
    let runtime = Builder::new_current_thread().worker_threads(1).enable_all().build().unwrap();

    runtime.block_on(async {
        let pool = setup_pool(&config.postgres).await;

        let state = AppState(Arc::new(AppStateInner {
            service: Service {
                fact: FactService::new(pool.clone(), ReadTradeRepo::new()),
                notification: NotificationService::new(pool.clone(), NotificationRepo::new()),
                strategy: StrategyService::new(pool.clone(), StrategyRepo::new()),
            },
        }));

        let strategies = state.service.strategy.list_active().await.unwrap();

        for (token_pair_id, facts) in state.service.fact.pumpfun_facts().await {
            for strategy in &strategies {
                if strategy.sequence.condition.test(&facts) {
                    println!("met - {token_pair_id}");
                    let mut tx = pool.begin().await.unwrap();

                    match InvocationRepo::new()
                        .create(
                            &mut tx,
                            InvocationCreateCmd {
                                user: strategy.user,
                                strategy: strategy.id,
                                token_pair: token_pair_id,
                                sequence: strategy.sequence.clone(),
                            },
                        )
                        .await
                    {
                        Ok(_) => {
                            match strategy.sequence.action {
                                Action::AndThen(_, _) => {}
                                Action::Buy => {}
                                Action::Notify => {
                                    let _ = state
                                        .service
                                        .notification
                                        .create_condition_met_tx(
                                            &mut tx,
                                            NotificationConditionMet {
                                                user: strategy.user,
                                                token_pair: token_pair_id,
                                            },
                                        )
                                        .await;
                                }
                                Action::Sell => {}
                            }

                            let _ = tx.commit().await.unwrap();
                        }
                        Err(_) => {
                            // FIXME cache already invoked strategies - otherwise this might be heavy on the database
                            let _ = tx.rollback().await.unwrap();
                        }
                    }
                }
            }
        }
        sleep(Duration::from_millis(1000)).await;
    })
}
