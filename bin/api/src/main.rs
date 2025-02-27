// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

#![cfg_attr(not(debug_assertions), deny(warnings))]

extern crate core;

use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;

use crate::config::Config;
use crate::http::state::{AppState, AppStateInner, Service};
use base::repo::{AuthRepo, RuleRepo};
use base::service::UserService;
use base::service::{AuthService, RuleService};
use common::repo::pool::setup_pool;
use common::ResolveOr;
use log::info;
use signal::unix::{signal, SignalKind};
use tokio::runtime::Builder;
use tokio::sync::broadcast;
use tokio::{join, signal};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

mod config;
mod http;
mod router;
mod ws;

fn main() {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // configure_panic_hook();

    let config = Config::load();

    let runtime = Builder::new_multi_thread().worker_threads(1).enable_all().build().unwrap();
    runtime.block_on(async {
        let (exit_sender, exit) = broadcast::channel(1);

        // let bind_addr: String = config
        //     .server
        //     .bind_address
        //     .resolve_or("127.0.0.1:6000".to_string());
        let bind_addr = format!("[::]:{}", config.server.port.resolve_or(8080));
        let bind_address = SocketAddr::from_str(&bind_addr).unwrap();

        let pool = setup_pool(&config.postgres).await;

        let router = router::setup_v1(AppState(Arc::new(AppStateInner {
            config,
            service: Service {
                auth: AuthService::new(pool.clone(), AuthRepo::new()),
                rule: RuleService::new(pool.clone(), RuleRepo::new()),
                user: UserService::new(pool.clone()),
            },
        })));

        tokio::spawn(async move {
            let mut sigterm = signal(SignalKind::terminate()).unwrap();
            tokio::select! {
                _ = sigterm.recv() => {
                    info!("Received SIGTERM. Cleaning up resources...");
                    exit_sender.send(()).expect("Unable to initiate shutdown");
                }
            }
        });

        let _ = join!(http::Server::start(bind_address, exit.resubscribe(), router).await,);
    })

    // // build our application with some routes
    // let app = Router::new()
    //     .layer(TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::default().include_headers(true)));
    //
    // // run it with hyper
    // let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
    //     .await
    //     .unwrap();
    //
    // info!("listening on {}", listener.local_addr().unwrap());
    // axum::serve(
    //     listener,
    //     app.into_make_service_with_connect_info::<SocketAddr>(),
    // )
    //     .await
    //     .unwrap();
}
