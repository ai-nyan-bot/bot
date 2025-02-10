// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use std::net::SocketAddr;

use axum::Router;
use log::{info, warn};
use tokio::task::JoinHandle;

pub struct Server {}

impl Server {
    pub async fn start(
        bind_address: SocketAddr,
        exit: tokio::sync::broadcast::Receiver<()>,
        router: Router,
    ) -> JoinHandle<()> {
        Server::new_server(bind_address, exit, router).await
    }
}

impl Server {
    async fn new_server(
        bind_address: SocketAddr,
        exit: tokio::sync::broadcast::Receiver<()>,
        router: Router,
    ) -> JoinHandle<()> {
        let listener = tokio::net::TcpListener::bind(bind_address).await.unwrap();
        let handle = axum::serve(
            listener,
            router.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .with_graceful_shutdown(Self::shutdown_signal(exit));

        info!("HTTP server started at {}", bind_address);
        tokio::spawn(async move {
            handle.await.expect("HTTP server failed");
        })
    }

    async fn shutdown_signal(mut exit: tokio::sync::broadcast::Receiver<()>) {
        exit.recv()
            .await
            .expect("listening to exit broadcast failed");
        warn!("shutting down HTTP server...");
    }
}
