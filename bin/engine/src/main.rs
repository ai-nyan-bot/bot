// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

#![cfg_attr(not(debug_assertions), deny(warnings))]

use crate::config::Config;
use crate::pumpfun::FactService;
use crate::state::{AppState, AppStateInner, Service};
use base::model::{Action, Venue};
use base::repo::{
    InvocationCreateCmd, InvocationRepo, NotificationRepo, RuleRepo, TokenPairRepo, TokenRepo,
};
use base::service::{NotificationRuleMatched, NotificationService, RuleService};
use common::repo::pool::setup_pool;
use solana::pumpfun::repo::{CurrentRepo, SummaryRepo};
use std::sync::Arc;
use std::time::Duration;
use tokio::runtime::Builder;
use tokio::time::{sleep, Instant};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;
use Venue::PumpFun;

mod config;
mod pumpfun;
mod state;

fn main() {
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::load();
    println!("Starting the engine");
    let runtime = Builder::new_current_thread()
        .worker_threads(1)
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
        let pool = setup_pool(&config.postgres).await;

        let token_repo = TokenRepo::new_read_only();

        let state = AppState(Arc::new(AppStateInner {
            service: Service {
                fact: FactService::new(
                    pool.clone(),
                    TokenPairRepo::new(token_repo.clone()),
                    SummaryRepo::new(),
                    CurrentRepo::new(),
                ),
                notification: NotificationService::new(pool.clone(), NotificationRepo::new()),
                rule: RuleService::new(pool.clone(), RuleRepo::new()),
            },
        }));

        loop {
            let rules = state.service.rule.list_active().await.unwrap();

            let start = Instant::now();
            let pumpfun_facts = state.service.fact.pumpfun_facts().await;
            println!(
                "{} pumpfun facts - took {}",
                pumpfun_facts.len(),
                (Instant::now().duration_since(start)).as_millis()
            );

            for rule in &rules {
                if !rule.applicable() {
                    // FIXME filter them out before hitting this loop
                    continue;
                }
                println!("test rule - {}", rule.id.0);
                for (token_pair_id, facts) in &pumpfun_facts {
                    if rule.sequence.condition.test(facts) {
                        let mut tx = pool.begin().await.unwrap();

                        match InvocationRepo::new()
                            .create(
                                &mut tx,
                                InvocationCreateCmd {
                                    user: rule.user,
                                    rule: rule.id,
                                    token_pair: *token_pair_id,
                                    next: None,
                                },
                            )
                            .await
                        {
                            Ok(_) => {
                                println!("met - {token_pair_id}");

                                match &rule.sequence.action {
                                    Action::AndThen { .. } => {}
                                    Action::Buy => {}
                                    Action::NotifyTelegram { buttons } => {
                                        let _ = state
                                            .service
                                            .notification
                                            .create_rule_matched_tx(
                                                &mut tx,
                                                NotificationRuleMatched::Telegram {
                                                    user: rule.user,
                                                    rule: rule.id,
                                                    venue: PumpFun,
                                                    token_pair: *token_pair_id,
                                                    buttons: buttons.clone(),
                                                },
                                            )
                                            .await;
                                    }
                                    Action::Sell => {}
                                }

                                tx.commit().await.unwrap();
                            }
                            Err(_) => {
                                // FIXME cache already invoked strategies - otherwise this might be heavy on the database
                                tx.rollback().await.unwrap();
                            }
                        }
                    }
                }
            }
            sleep(Duration::from_millis(1000)).await;
        }
    })
}
